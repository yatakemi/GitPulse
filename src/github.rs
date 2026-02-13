use anyhow::{anyhow, Result, Context};
use std::process::Command;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubReview {
    pub user: String,
    pub state: String,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubReviewComment {
    pub user: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubPR {
    pub number: u32,
    pub title: String,
    pub author: String,
    pub html_url: String,
    #[serde(default = "default_datetime")]
    pub created_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub additions: usize,
    #[serde(default)]
    pub deletions: usize,
    #[serde(default)]
    pub changed_files: usize,
    #[serde(default)]
    pub total_comments: usize,
    pub reviews: Vec<GitHubReview>,
    pub review_requests: Vec<String>,
    pub review_comments: Vec<GitHubReviewComment>,
}

fn default_datetime() -> DateTime<Utc> {
    Utc.timestamp_opt(0, 0).unwrap()
}

pub struct GitHubClient {
    token: String,
    repo: String, // "owner/repo"
    agent: ureq::Agent,
    cache_dir: std::path::PathBuf,
}

impl GitHubClient {
    pub fn new(repo_path: &std::path::Path, cache_dir: &std::path::Path) -> Result<Self> {
        let token = Self::get_token()?;
        let repo = Self::get_repo_name(repo_path)?;
        
        // Setup proxy from environment
        let mut agent_builder = ureq::AgentBuilder::new();
        if let Ok(proxy_url) = std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("https_proxy")) {
            agent_builder = agent_builder.proxy(ureq::Proxy::new(proxy_url)?);
        } else if let Ok(proxy_url) = std::env::var("HTTP_PROXY").or_else(|_| std::env::var("http_proxy")) {
            agent_builder = agent_builder.proxy(ureq::Proxy::new(proxy_url)?);
        }
        
        Ok(Self { 
            token, 
            repo,
            agent: agent_builder.build(),
            cache_dir: cache_dir.to_path_buf(),
        })
    }

    pub fn get_cache_path(&self) -> std::path::PathBuf {
        let mut path = self.cache_dir.clone();
        let safe_repo = self.repo.replace('/', "_");
        path.push(format!(".gitpulse_cache_{}.json", safe_repo));
        path
    }

    pub fn load_cache(&self) -> Option<Vec<GitHubPR>> {
        let path = self.get_cache_path();
        if path.exists() {
            if let Ok(file) = std::fs::File::open(path) {
                if let Ok(prs) = serde_json::from_reader(file) {
                    println!("📦 Using cached GitHub data...");
                    return Some(prs);
                }
            }
        }
        None
    }

    pub fn save_cache(&self, prs: &[GitHubPR]) -> Result<()> {
        let path = self.get_cache_path();
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, prs)?;
        Ok(())
    }

    fn get_token() -> Result<String> {
        // 1. Try gh auth token
        if let Ok(output) = Command::new("gh").args(["auth", "token"]).output() {
            if output.status.success() {
                let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !token.is_empty() {
                    return Ok(token);
                }
            }
        }

        // 2. Try environment variables
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            return Ok(token);
        }
        if let Ok(token) = std::env::var("GH_TOKEN") {
            return Ok(token);
        }

        Err(anyhow!("GitHub token not found. Please login with 'gh auth login' or set GITHUB_TOKEN environment variable."))
    }

    fn get_repo_name(repo_path: &std::path::Path) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .args(["remote", "get-url", "origin"])
            .output()
            .context("Failed to execute git remote get-url origin")?;

        if !output.status.success() {
            return Err(anyhow!("Failed to get git remote URL. Is this a git repository with an 'origin' remote?"));
        }

        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        // Handle SSH: git@github.com:owner/repo.git
        // Handle HTTPS: https://github.com/owner/repo.git
        if url.contains("github.com") {
            let path = url.split("github.com").last().unwrap();
            let path = path.trim_start_matches(':').trim_start_matches('/');
            let path = path.trim_end_matches(".git");
            Ok(path.to_string())
        } else {
            Err(anyhow!("Remote URL is not a GitHub URL: {}", url))
        }
    }

    pub fn fetch_reviews(&self) -> Result<Vec<GitHubPR>> {
        println!("🔍 Fetching GitHub PRs and reviews for {}...", self.repo);
        
        let mut all_prs = Vec::new();
        let mut cursor: Option<String> = None;
        let pages_to_fetch = 10; // Fetch up to 500 PRs (50 per page)

        for page in 1..=pages_to_fetch {
            if page > 1 {
                print!("\r   Fetching page {}/{}...", page, pages_to_fetch);
                use std::io::Write;
                std::io::stdout().flush().ok();
            }

            // Using GraphQL for efficiency to get PRs and their reviews in one go
            let query = r#"
            query($owner: String!, $name: String!, $cursor: String) {
              repository(owner: $owner, name: $name) {
                pullRequests(last: 50, before: $cursor, states: [OPEN, MERGED, CLOSED]) {
                  pageInfo {
                    hasPreviousPage
                    startCursor
                  }
                  nodes {
                    number
                    title
                    url
                    author { login }
                    createdAt
                    mergedAt
                    state
                    additions
                    deletions
                    changedFiles
                    comments { totalCount }
                    reviewRequests(last: 20) {
                      nodes {
                        requestedReviewer {
                          ... on User { login }
                          ... on Team { name }
                        }
                      }
                    }
                    reviews(last: 50) {
                      nodes {
                        author { login }
                        state
                        submittedAt
                      }
                    }
                    reviewThreads(last: 50) {
                      nodes {
                        comments(first: 1) {
                          nodes {
                            author { login }
                            createdAt
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            "#;

            let parts: Vec<&str> = self.repo.split('/').collect();
            let owner = parts[0];
            let name = parts[1];

            let response: serde_json::Value = self.agent.post("https://api.github.com/graphql")
                .set("Authorization", &format!("Bearer {}", self.token))
                .set("User-Agent", "GitPulse")
                .send_json(serde_json::json!({
                    "query": query,
                    "variables": { "owner": owner, "name": name, "cursor": cursor }
                }))?
                .into_json()?;

            if let Some(errors) = response.get("errors") {
                return Err(anyhow!("GitHub API error: {}", errors));
            }

            let pr_data = &response["data"]["repository"]["pullRequests"];
            if let Some(nodes) = pr_data["nodes"].as_array() {
                for node in nodes {
                    let mut reviews = Vec::new();
                    if let Some(review_nodes) = node["reviews"]["nodes"].as_array() {
                        for r_node in review_nodes {
                            if let (Some(author), Some(state), Some(submitted_at)) = (
                                r_node["author"]["login"].as_str(),
                                r_node["state"].as_str(),
                                r_node["submittedAt"].as_str()
                            ) {
                                reviews.push(GitHubReview {
                                    user: author.to_string(),
                                    state: state.to_string(),
                                    submitted_at: DateTime::parse_from_rfc3339(submitted_at)?.with_timezone(&Utc),
                                });
                            }
                        }
                    }

                    let mut review_requests = Vec::new();
                    if let Some(req_nodes) = node["reviewRequests"]["nodes"].as_array() {
                        for req_node in req_nodes {
                            if let Some(login) = req_node["requestedReviewer"]["login"].as_str() {
                                review_requests.push(login.to_string());
                            } else if let Some(name) = req_node["requestedReviewer"]["name"].as_str() {
                                review_requests.push(name.to_string());
                            }
                        }
                    }

                    let mut review_comments = Vec::new();
                    if let Some(thread_nodes) = node["reviewThreads"]["nodes"].as_array() {
                        for thread_node in thread_nodes {
                            if let Some(comment_nodes) = thread_node["comments"]["nodes"].as_array() {
                                if let Some(first_comment) = comment_nodes.get(0) {
                                    if let (Some(author), Some(created_at)) = (
                                        first_comment["author"]["login"].as_str(),
                                        first_comment["createdAt"].as_str()
                                    ) {
                                        review_comments.push(GitHubReviewComment {
                                            user: author.to_string(),
                                            created_at: DateTime::parse_from_rfc3339(created_at)?.with_timezone(&Utc),
                                        });
                                    }
                                }
                            }
                        }
                    }

                    all_prs.push(GitHubPR {
                        number: node["number"].as_u64().unwrap_or(0) as u32,
                        title: node["title"].as_str().unwrap_or("").to_string(),
                        author: node["author"]["login"].as_str().unwrap_or("").to_string(),
                        html_url: node["url"].as_str().unwrap_or("").to_string(),
                        created_at: DateTime::parse_from_rfc3339(node["createdAt"].as_str().unwrap_or(""))?.with_timezone(&Utc),
                        merged_at: node["mergedAt"].as_str().and_then(|s| DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc))),
                        state: node["state"].as_str().unwrap_or("").to_string(),
                        additions: node["additions"].as_u64().unwrap_or(0) as usize,
                        deletions: node["deletions"].as_u64().unwrap_or(0) as usize,
                        changed_files: node["changedFiles"].as_u64().unwrap_or(0) as usize,
                        total_comments: node["comments"]["totalCount"].as_u64().unwrap_or(0) as usize,
                        reviews,
                        review_requests,
                        review_comments,
                    });
                }
            }

            let page_info = &pr_data["pageInfo"];
            if page_info["hasPreviousPage"].as_bool().unwrap_or(false) {
                cursor = page_info["startCursor"].as_str().map(|s| s.to_string());
            } else {
                break;
            }
        }

        if pages_to_fetch > 1 { println!(); }
        Ok(all_prs)
    }
}
