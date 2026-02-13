use anyhow::{Context, Result};
use chrono::Timelike;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use tera::{Context as TeraContext, Tera};

#[derive(Debug, Serialize, Clone)]
pub struct AggregatedStats {
    pub date: String,
    pub user: String,
    pub added: usize,
    pub deleted: usize,
    pub total_changes: usize,
    pub commit_count: usize,
}

pub fn visualize_stats(input_path: &Path, output_path: &Path, format: &str) -> Result<()> {
    let file = File::open(input_path).context("Failed to open input file")?;
    let reader = BufReader::new(file);
    // Parse as ReportData instead of Vec<CommitStats>
    let mut report_data: crate::model::ReportData = serde_json::from_reader(reader).context("Failed to parse JSON")?;
    
    // Resize (Normalize) authors in commits
    // Load config from current directory or repository root
    let config_path = Path::new("gitpulse.toml");
    let config = if config_path.exists() {
        crate::config::Config::load(config_path)
            .context(format!("Failed to parse config file: {:?}", config_path))?
    } else {
        crate::config::Config::default()
    };

    for commit in &mut report_data.commits {
        commit.author = normalize_author(&commit.author, &commit.email, &config);
    }

    // Aggregate data by day and user: (added, deleted, commit_count)
    let mut aggregation: HashMap<(String, String), (usize, usize, usize)> = HashMap::new();

    for commit in &report_data.commits {
        // Group by day
        let date = commit.date.date_naive();
        let day_str = date.format("%Y-%m-%d").to_string();

        let key = (day_str, commit.author.clone());
        let entry = aggregation.entry(key).or_insert((0, 0, 0));
        entry.0 += commit.added;
        entry.1 += commit.deleted;
        entry.2 += 1; // Increment commit count
    }

    let mut stats_list: Vec<AggregatedStats> = aggregation
        .into_iter()
        .map(|((date, user), (added, deleted, commit_count))| AggregatedStats {
            date,
            user,
            added,
            deleted,
            total_changes: added + deleted,
            commit_count,
        })
        .collect();

    // Sort by date
    stats_list.sort_by(|a, b| a.date.cmp(&b.date));

    match format {
        "csv" => export_csv(&stats_list, output_path),
        "html" => export_html(&report_data, output_path),
        _ => anyhow::bail!("Unsupported format: {}", format),
    }
}

fn normalize_author(name: &str, email: &str, config: &crate::config::Config) -> String {
    // 1. Check alias by email
    if let Some(alias) = config.alias.get(email) {
        return alias.clone();
    }

    // 2. Check alias by name
    if let Some(alias) = config.alias.get(name) {
        return alias.clone();
    }

    // 3. GitHub noreply auto-merge
    // Format: 12345+username@users.noreply.github.com
    if email.ends_with("@users.noreply.github.com") {
        if let Some(local_part) = email.split('@').next() {
            if let Some(plus_pos) = local_part.find('+') {
                return local_part[plus_pos + 1..].to_string();
            }
        }
    }

    // Default: return original name
    name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_normalize_author() {
        let mut alias = HashMap::new();
        alias.insert("alice@example.com".to_string(), "Alice".to_string());
        alias.insert("Bob_Work".to_string(), "Bob".to_string());
        
        let mut config = crate::config::Config::default();
        config.alias = alias;

        // Alias by email
        assert_eq!(normalize_author("Alice P", "alice@example.com", &config), "Alice");
        
        // Alias by name
        assert_eq!(normalize_author("Bob_Work", "bob@corp.com", &config), "Bob");
        
        // GitHub noreply
        assert_eq!(normalize_author("yatakemi", "12345+yatakemi@users.noreply.github.com", &config), "yatakemi");
        assert_eq!(normalize_author("unknown", "999+someone@users.noreply.github.com", &config), "someone");

        // No match
        assert_eq!(normalize_author("Charlie", "charlie@gmail.com", &config), "Charlie");
    }
}

fn export_csv(stats: &[AggregatedStats], output_path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(output_path)?;
    for stat in stats {
        wtr.serialize(stat)?;
    }
    wtr.flush()?;
    println!("Exported CSV to {:?}", output_path);
    Ok(())
}

fn export_html(data: &crate::model::ReportData, output_path: &Path) -> Result<()> {
    let config_path = Path::new("gitpulse.toml");
    let config = if config_path.exists() {
        crate::config::Config::load(config_path).unwrap_or_default()
    } else {
        crate::config::Config::default()
    };

    // Check for GitHub users who don't match any local users
    let mut local_users = std::collections::HashSet::new();
    for commit in &data.commits {
        local_users.insert(commit.author.clone());
    }

    let mut unmapped_github_users = std::collections::HashSet::new();
    for pr in &data.github_prs {
        for review in &pr.reviews {
            // Apply normalization/alias to the GitHub username
            let normalized = normalize_author(&review.user, "", &config);
            if !local_users.contains(&normalized) {
                unmapped_github_users.insert(review.user.clone());
            }
        }
    }

    if !unmapped_github_users.is_empty() {
        println!("⚠️  Found GitHub reviewers not mapped to local users:");
        let mut sorted_users: Vec<_> = unmapped_github_users.into_iter().collect();
        sorted_users.sort();
        for user in sorted_users {
            println!("   - {}", user);
        }
        println!("   (Tip: Add these to [alias] in gitpulse.toml to link them to local users)");
    }

    let dashboard_data = aggregate_dashboard_data(data, &config);
    
    let mut context = TeraContext::new();
    context.insert("data", &dashboard_data);
    context.insert("aliases", &config.alias);
    let rendered = Tera::one_off(crate::html_template::HTML_TEMPLATE, &context, false)
        .context("Failed to render HTML template")?;
    let mut file = File::create(output_path)?;
    file.write_all(rendered.as_bytes())?;
    println!("Exported HTML to {:?}", output_path);
    Ok(())
}

fn aggregate_dashboard_data(data: &crate::model::ReportData, config: &crate::config::Config) -> crate::model::DashboardData {
    use std::collections::{HashMap, HashSet};
    use crate::model::{DailyStat, FileStat, MergeEvent, WeeklyStat};

    let mut daily_map: HashMap<(String, String), DailyStat> = HashMap::new();
    let mut file_map: HashMap<(usize, String), usize> = HashMap::new();
    let mut merge_events = Vec::new();
    let mut daily_dirs: HashMap<String, HashSet<String>> = HashMap::new();
    let mut weekly_map: HashMap<String, WeeklyStat> = HashMap::new();
    let mut ext_map: HashMap<String, crate::model::FileTypeStat> = HashMap::new();
    let mut daily_ext_map: HashMap<(String, String), (usize, usize)> = HashMap::new();
    let mut daily_lead_time_map: HashMap<String, Vec<f64>> = HashMap::new();
    let mut daily_user_prs: HashMap<(String, String), HashSet<u32>> = HashMap::new();

    // Grouping commits for merge time calculation
    let mut non_merge_commits = data.commits.clone();
    non_merge_commits.retain(|c| !c.is_merge);
    non_merge_commits.sort_by(|a, b| a.date.cmp(&b.date));
    
    for commit in &data.commits {
        let date_str = commit.date.date_naive().format("%Y-%m-%d").to_string();
        let hour = commit.date.hour();
        let total = commit.added + commit.deleted;
        let churn = (commit.added + commit.deleted) as i64 - (commit.added as i64 - commit.deleted as i64).abs();
        let churn = churn as usize;

        // Monday-based week start
        use chrono::Datelike;
        let weekday = commit.date.weekday().num_days_from_monday();
        let week_start = commit.date.date_naive() - chrono::Duration::days(weekday as i64);
        let week_start_str = week_start.format("%Y-%m-%d").to_string();

        // Daily stats
        let key = (date_str.clone(), commit.author.clone());
        let stat = daily_map.entry(key.clone()).or_insert(DailyStat {
            date: date_str.clone(),
            author: commit.author.clone(),
            added: 0,
            deleted: 0,
            commits: 0,
            merges: 0,
            churn: 0,
            hours: Vec::new(),
            commit_sizes: Vec::new(),
            unrelated_switches: 0,
            commit_intervals: Vec::new(),
            active_prs: 0,
        });
        stat.added += commit.added;
        stat.deleted += commit.deleted;
        stat.commits += 1;
        if commit.is_merge {
            stat.merges += 1;
        }
        stat.churn += churn;
        stat.hours.push(hour);
        stat.commit_sizes.push(total);

        if let Some(pr_num) = commit.pr_number {
            daily_user_prs.entry(key.clone()).or_insert(HashSet::new()).insert(pr_num);
        }

        // Weekly stats (global)
        let w_stat = weekly_map.entry(week_start_str.clone()).or_insert(WeeklyStat {
            week_start: week_start_str,
            added: 0,
            deleted: 0,
            commits: 0,
            churn: 0,
        });
        w_stat.added += commit.added;
        w_stat.deleted += commit.deleted;
        w_stat.commits += 1;
        w_stat.churn += churn;

        // File stats and directory tracking
        let day_dir_set = daily_dirs.entry(date_str.clone()).or_insert(HashSet::new());
        let mut commit_exts = HashSet::new();
        for &file_idx in &commit.files {
            if file_idx >= data.file_paths.len() { continue; } // Safety check
            
            let file_key = (file_idx, commit.author.clone());
            *file_map.entry(file_key).or_insert(0) += 1;

            if let Some(path) = data.file_paths.get(file_idx) {
                let dir = if let Some(pos) = path.find('/') {
                    &path[..pos]
                } else {
                    "(root)"
                };
                day_dir_set.insert(dir.to_string());

                // Extract extension
                let path_lower = path.to_lowercase();
                let filename = path.split('/').last().unwrap_or("").to_lowercase();
                
                let ext = if path_lower.contains("/test/") || path_lower.contains("/tests/") || 
                            filename.contains(".spec.") || filename.contains(".test.") ||
                            filename.ends_with("_test.rs") || filename.ends_with("_spec.rb") ||
                            filename.starts_with("test_")
                {
                    "test".to_string()
                } else {
                    let e = path.split('.').last().unwrap_or("no-ext").to_lowercase();
                    if e.len() < 8 && !path.ends_with('/') {
                        e
                    } else if !path.contains('.') {
                        "no-ext".to_string()
                    } else {
                        "others".to_string()
                    }
                };
                commit_exts.insert(ext);
            }
        }

        // Aggregate by extension
        let num_exts = commit_exts.len().max(1);
        for ext in commit_exts {
            let added_share = commit.added / num_exts;
            let deleted_share = commit.deleted / num_exts;

            let stat = ext_map.entry(ext.clone()).or_insert(crate::model::FileTypeStat {
                extension: ext.clone(),
                added: 0,
                deleted: 0,
                commits: 0,
                churn: 0,
            });
            stat.added += added_share;
            stat.deleted += deleted_share;
            stat.commits += 1;
            stat.churn += churn / num_exts;

            let daily_ext_stat = daily_ext_map.entry((date_str.clone(), ext)).or_insert((0, 0));
            daily_ext_stat.0 += added_share;
            daily_ext_stat.1 += deleted_share;
        }

        // Merge events
        if commit.is_merge {
            // Priority 1: Use pre-calculated lead time if available
            // Priority 2: Use regex to find branch name
            let branch_name = if let Some(branch_match) = regex::Regex::new(r"(?i)(?:Merge\s+(?:branch|pull request)\s+'?([^'\s]+)'?|#(\d+))")
                .ok()
                .and_then(|re| re.captures(&commit.message))
            {
                if let Some(m) = branch_match.get(1) {
                    m.as_str().to_string()
                } else if let Some(m) = branch_match.get(2) {
                    format!("PR #{}", m.as_str())
                } else {
                    "Unknown Branch".to_string()
                }
            } else {
                format!("Merge {}", &commit.hash[..7])
            };

            let days = commit.lead_time_days.unwrap_or_else(|| {
                // Fallback to simple approximation if for some reason it's missing
                if let Some(pos) = non_merge_commits.iter().rposition(|c| c.date < commit.date) {
                    let pred = &non_merge_commits[pos];
                    let duration = commit.date.signed_duration_since(pred.date);
                    duration.num_seconds() as f64 / (24.0 * 3600.0)
                } else {
                    1.0
                }
            });

            if days <= 365.0 { // Support up to 1 year lead time
                merge_events.push(MergeEvent {
                    branch: branch_name,
                    author: commit.author.clone(),
                    days,
                    date: date_str.clone(),
                });
                daily_lead_time_map.entry(date_str).or_insert(Vec::new()).push(days);
            }
        }
    }

    let daily_dir_counts = daily_dirs.into_iter()
        .map(|(date, dirs)| crate::model::DirCount { date, count: dirs.len() })
        .collect();

    let mut weekly_stats: Vec<WeeklyStat> = weekly_map.into_values().collect();
    weekly_stats.sort_by(|a, b| a.week_start.cmp(&b.week_start));

    // Simple Forecasting Logic
    let forecast = if weekly_stats.len() >= 2 {
        let last_4_weeks: Vec<&WeeklyStat> = weekly_stats.iter().rev().take(4).collect();
        let current_velocity = last_4_weeks.iter().map(|w| w.commits as f64).sum::<f64>() / last_4_weeks.len() as f64;
        
        // Trend: compare last 2 weeks vs previous 2 weeks
        let v_recent = if last_4_weeks.len() >= 2 {
            (last_4_weeks[0].commits + last_4_weeks[1].commits) as f64 / 2.0
        } else {
            last_4_weeks[0].commits as f64
        };
        let v_prev = if last_4_weeks.len() >= 4 {
            (last_4_weeks[2].commits + last_4_weeks[3].commits) as f64 / 2.0
        } else if last_4_weeks.len() >= 3 {
            last_4_weeks[2].commits as f64
        } else {
            v_recent // no trend if data is too small
        };
        
        let velocity_trend = if v_prev > 0.0 {
            ((v_recent - v_prev) / v_prev) * 100.0
        } else {
            0.0
        };

        let projected_60_days = (current_velocity * (60.0 / 7.0)) as usize;

        Some(crate::model::ForecastData {
            current_velocity,
            velocity_trend,
            projected_60_days,
            est_completion_date: None, // Will calculate in JS with dynamic target
        })
    } else {
        None
    };

    // --- Detailed Context Switch Analysis ---
    // Group commits by (date, author) to analyze chronological flow
    let mut user_day_commits: HashMap<(String, String), Vec<crate::model::CommitStats>> = HashMap::new();
    for commit in &data.commits {
        let date_str = commit.date.date_naive().format("%Y-%m-%d").to_string();
        user_day_commits.entry((date_str, commit.author.clone())).or_insert(Vec::new()).push(commit.clone());
    }

    for ((date_str, author), mut commits) in user_day_commits {
        if commits.len() < 2 { continue; }
        commits.sort_by(|a, b| a.date.cmp(&b.date));

        let mut last_dirs: HashSet<String> = HashSet::new();
        let mut last_time = commits[0].date;

        for (i, commit) in commits.iter().enumerate() {
            let current_dirs: HashSet<String> = commit.files.iter()
                .filter_map(|&idx| data.file_paths.get(idx))
                .map(|p| p.split('/').next().unwrap_or("(root)").to_string())
                .collect();

            if i > 0 {
                // 1. Time Fragmentation
                let diff_min = commit.date.signed_duration_since(last_time).num_minutes() as u32;
                if let Some(stat) = daily_map.get_mut(&(date_str.clone(), author.clone())) {
                    stat.commit_intervals.push(diff_min);
                }

                // 2. Switch Quality
                // A switch is unrelated if there's no intersection between current dirs and last dirs,
                // AND it's not a src <-> test switch.
                if !current_dirs.is_empty() && !last_dirs.is_empty() {
                    let has_intersection = current_dirs.iter().any(|d| last_dirs.contains(d));
                    if !has_intersection {
                        let is_src_test_switch = (current_dirs.iter().any(|d| d == "src") && last_dirs.iter().any(|d| d == "test" || d == "tests" || d == "spec")) ||
                                               (last_dirs.iter().any(|d| d == "src") && current_dirs.iter().any(|d| d == "test" || d == "tests" || d == "spec"));
                        
                        if !is_src_test_switch {
                            if let Some(stat) = daily_map.get_mut(&(date_str.clone(), author.clone())) {
                                stat.unrelated_switches += 1;
                            }
                        }
                    }
                }
            }
            last_dirs = current_dirs;
            last_time = commit.date;
        }
    }

    // Assign active PR counts
    for ((date_str, user), prs) in daily_user_prs {
        if let Some(stat) = daily_map.get_mut(&(date_str, user)) {
            stat.active_prs = prs.len();
        }
    }

    let daily_file_type_stats = daily_ext_map.into_iter()
        .map(|((date, extension), (added, deleted))| crate::model::DailyFileTypeStat { date, extension, added, deleted })
        .collect();

    let daily_lead_time_stats = daily_lead_time_map.into_iter()
        .map(|(date, days)| {
            let avg_days = days.iter().sum::<f64>() / days.len() as f64;
            crate::model::DailyLeadTimeStat { date, avg_days }
        })
        .collect();

    crate::model::DashboardData {
        daily_stats: daily_map.into_values().collect(),
        file_stats: file_map.into_iter().map(|((f, a), c)| FileStat { file_idx: f, author: a, count: c }).collect(),
        file_type_stats: ext_map.into_values().collect(),
        daily_file_type_stats,
        daily_lead_time_stats,
        merge_events,
        daily_dir_counts,
        weekly_stats,
        forecast,
        github_prs: data.github_prs.clone(),
        file_paths: data.file_paths.clone(),
        commits: data.commits.clone(),
        events: config.events.clone(),
    }
}
