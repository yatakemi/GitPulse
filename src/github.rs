use anyhow::{anyhow, Result, Context};
use std::process::Command;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubReview {
    pub user: String,
    pub state: String,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubPR {
    pub number: u32,
    pub title: String,
    pub author: String,
    pub html_url: String,
    pub reviews: Vec<GitHubReview>,
}

pub struct GitHubClient {
    token: String,
    repo: String, // "owner/repo"
}

impl GitHubClient {
    pub fn new(repo_path: &std::path::Path) -> Result<Self> {
        let token = Self::get_token()?;
        let repo = Self::get_repo_name(repo_path)?;
        Ok(Self { token, repo })
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
        
        // Using GraphQL for efficiency to get PRs and their reviews in one go
        let query = r#"
        query($owner: String!, $name: String!) {
          repository(owner: $owner, name: $name) {
            pullRequests(last: 100, states: [OPEN, MERGED, CLOSED]) {
              nodes {
                number
                title
                url
                author { login }
                reviews(last: 50) {
                  nodes {
                    author { login }
                    state
                    submittedAt
                  }
                }
              }
            }
          }
        }
        "#;

        let parts: Vec<&str> = self.repo.split('/').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid repo format: {}", self.repo));
        }
        let owner = parts[0];
        let name = parts[1];

        let response: serde_json::Value = ureq::post("https://api.github.com/graphql")
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("User-Agent", "GitPulse")
            .send_json(serde_json::json!({
                "query": query,
                "variables": { "owner": owner, "name": name }
            }))?
            .into_json()?;

        if let Some(errors) = response.get("errors") {
            return Err(anyhow!("GitHub API error: {}", errors));
        }

        let mut prs = Vec::new();
        if let Some(nodes) = response["data"]["repository"]["pullRequests"]["nodes"].as_array() {
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

                prs.push(GitHubPR {
                    number: node["number"].as_u64().unwrap_or(0) as u32,
                    title: node["title"].as_str().unwrap_or("").to_string(),
                    author: node["author"]["login"].as_str().unwrap_or("").to_string(),
                    html_url: node["url"].as_str().unwrap_or("").to_string(),
                    reviews,
                });
            }
        }

        Ok(prs)
    }
}
