// Example implementations of SimpleFormattable for protocol types
// This shows how easy it is to add beautiful formatting for any data type

use crate::display::{utils, CollectionFormattable, CompactTableFormattable, SimpleFormattable};
use nexsock_protocol::commands::{
    config::ServiceConfigPayload,
    dependency::ListDependenciesResponse,
    error::ErrorPayload,
    git::{GitCommitInfo, GitListBranchesResponse, GitLogResponse, RepoStatus},
    list_services::{ListServicesResponse, ServiceInfo},
    service_status::ServiceStatus,
};

impl SimpleFormattable for ServiceStatus {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![
            ("ID".to_string(), self.id.to_string()),
            ("Name".to_string(), self.name.clone()),
            ("Status".to_string(), format!("{:?}", self.state)),
            ("Port".to_string(), self.port.to_string()),
            ("Repository URL".to_string(), self.repo_url.clone()),
            ("Repository Path".to_string(), self.repo_path.clone()),
        ];
        
        if let Some(ref branch) = self.git_branch {
            pairs.push(("Git Branch".to_string(), branch.clone()));
        }
        
        if let Some(ref commit) = self.git_commit_hash {
            pairs.push(("Git Commit".to_string(), commit[..7.min(commit.len())].to_string()));
        }
        
        if let Some(ref config) = self.config {
            if let Some(ref filename) = config.filename {
                pairs.push(("Config File".to_string(), filename.clone()));
            }
            if let Some(ref run_command) = config.run_command {
                pairs.push(("Run Command".to_string(), run_command.clone()));
            }
        }
        
        if !self.dependencies.is_empty() {
            let deps: Vec<String> = self.dependencies.iter()
                .map(|d| format!("{} ({:?})", d.name, d.state))
                .collect();
            pairs.push(("Dependencies".to_string(), deps.join(", ")));
        }
        
        pairs
    }
    
    fn title(&self) -> Option<String> {
        Some(format!("Service Details: {}", self.name))
    }
}

impl SimpleFormattable for ErrorPayload {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![
            ("Error Code".to_string(), self.code.to_string()),
            ("Message".to_string(), self.message.clone()),
        ];
        
        if let Some(ref details) = self.details {
            pairs.push(("Details".to_string(), details.clone()));
        }
        
        pairs
    }
    
    fn title(&self) -> Option<String> {
        Some("Error".to_string())
    }
}

// ServiceInfo implementation for individual services
impl SimpleFormattable for ServiceInfo {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("ID".to_string(), self.id.to_string()),
            ("Name".to_string(), self.name.clone()),
            ("Status".to_string(), format!("{:?}", self.state)),
            ("Port".to_string(), self.port.to_string()),
            ("Has Dependencies".to_string(), if self.has_dependencies { "Yes" } else { "No" }.to_string()),
        ]
    }
    
    fn title(&self) -> Option<String> {
        Some(format!("Service: {}", self.name))
    }
}

// For collections like ListServicesResponse, implement both summary and detailed views
impl SimpleFormattable for ListServicesResponse {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        if self.services.is_empty() {
            vec![("Status".to_string(), "No services found".to_string())]
        } else {
            let mut pairs = vec![
                ("Total Services".to_string(), self.services.len().to_string()),
            ];
            
            // Group by status
            let mut status_counts = std::collections::HashMap::new();
            for service in &self.services {
                *status_counts.entry(format!("{:?}", service.state)).or_insert(0) += 1;
            }
            
            for (status, count) in status_counts {
                pairs.push((format!("{} Services", status), count.to_string()));
            }
            
            pairs
        }
    }
    
    fn title(&self) -> Option<String> {
        Some("Services Overview".to_string())
    }
}

impl CollectionFormattable for ListServicesResponse {
    type Item = ServiceInfo;
    
    fn items(&self) -> &[Self::Item] {
        &self.services
    }
}

impl CompactTableFormattable for ListServicesResponse {
    fn table_headers(&self) -> Vec<&'static str> {
        vec!["ID", "Name", "Status", "Port", "Dependencies"]
    }
    
    fn table_rows(&self) -> Vec<Vec<String>> {
        self.services.iter().map(|service| {
            vec![
                service.id.to_string(),
                service.name.clone(),
                format!("{:?}", service.state),
                service.port.to_string(),
                if service.has_dependencies { "Yes" } else { "No" }.to_string(),
            ]
        }).collect()
    }
    
    fn table_title(&self) -> Option<String> {
        Some("Services".to_string())
    }
}

// Helper macro to quickly implement SimpleFormattable for simple types
#[macro_export]
macro_rules! impl_simple_formattable {
    ($type:ty, $title:expr) => {
        impl $crate::display::SimpleFormattable for $type {
            fn to_key_value_pairs(&self) -> Vec<(String, String)> {
                // Use debug representation as fallback
                vec![("Data".to_string(), format!("{:?}", self))]
            }
            
            fn title(&self) -> Option<String> {
                Some($title.to_string())
            }
        }
    };
}

// Git commit info for individual commits
impl SimpleFormattable for GitCommitInfo {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("Hash".to_string(), self.short_hash.clone()),
            ("Author".to_string(), format!("{} <{}>", self.author_name, self.author_email)),
            ("Date".to_string(), utils::format_timestamp(&self.timestamp)),
            ("Message".to_string(), self.message.clone()),
        ]
    }
    
    fn title(&self) -> Option<String> {
        Some(format!("Commit {}", self.short_hash))
    }
}

// Git log response with detailed commit info
impl SimpleFormattable for GitLogResponse {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("Total Commits".to_string(), self.commits.len().to_string()),
            ("Latest Commit".to_string(), 
                self.commits.first()
                    .map(|c| format!("{} - {}", c.short_hash, c.message))
                    .unwrap_or_else(|| "None".to_string())),
        ]
    }
    
    fn title(&self) -> Option<String> {
        Some("Git History".to_string())
    }
}

impl CollectionFormattable for GitLogResponse {
    type Item = GitCommitInfo;
    
    fn items(&self) -> &[Self::Item] {
        &self.commits
    }
    
    fn should_show_detailed(&self) -> bool {
        false // Changed to prefer compact view by default
    }
}

impl CompactTableFormattable for GitLogResponse {
    fn table_headers(&self) -> Vec<&'static str> {
        vec!["Hash", "Author", "Date", "Message"]
    }
    
    fn table_rows(&self) -> Vec<Vec<String>> {
        self.commits.iter().map(|commit| {
            vec![
                commit.short_hash.clone(),
                format!("{} <{}>", commit.author_name, commit.author_email),
                utils::format_timestamp(&commit.timestamp),
                utils::truncate_string(&commit.message, 60),
            ]
        }).collect()
    }
    
    fn table_title(&self) -> Option<String> {
        Some("Git History".to_string())
    }
}

// Git branches
impl SimpleFormattable for GitListBranchesResponse {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("Total Branches".to_string(), self.branches.len().to_string()),
            ("Branches".to_string(), self.branches.join(", ")),
        ]
    }
    
    fn title(&self) -> Option<String> {
        Some("Git Branches".to_string())
    }
}

// Repository status
impl SimpleFormattable for RepoStatus {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = vec![
            ("Remote URL".to_string(), self.remote_url.clone()),
            ("Current Commit".to_string(), utils::format_commit_hash(&self.current_commit, true).to_string()),
            ("Working Directory".to_string(), if self.is_dirty { "Modified" } else { "Clean" }.to_string()),
        ];
        
        if let Some(ref branch) = self.current_branch {
            pairs.push(("Current Branch".to_string(), branch.clone()));
        }
        
        if let (Some(ahead), Some(behind)) = (self.ahead_count, self.behind_count) {
            if ahead > 0 || behind > 0 {
                pairs.push(("Sync Status".to_string(), format!("{} ahead, {} behind", ahead, behind)));
            } else {
                pairs.push(("Sync Status".to_string(), "Up to date".to_string()));
            }
        }
        
        pairs
    }
    
    fn title(&self) -> Option<String> {
        Some("Repository Status".to_string())
    }
}

// Service configuration
impl SimpleFormattable for ServiceConfigPayload {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("Service".to_string(), self.service.to_string()),
            ("Config File".to_string(), self.filename.clone()),
            ("Format".to_string(), format!("{:?}", self.format)),
            ("Run Command".to_string(), self.run_command.clone()),
        ]
    }
    
    fn title(&self) -> Option<String> {
        Some(format!("Configuration for {}", self.service))
    }
}

// Dependencies response  
impl SimpleFormattable for ListDependenciesResponse {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        if self.dependencies.is_empty() {
            vec![("Dependencies".to_string(), format!("No dependencies for '{}'", self.service_name))]
        } else {
            let deps: Vec<String> = self.dependencies.iter()
                .map(|d| format!("{} ({:?}{})", 
                    d.name, 
                    d.state,
                    if d.tunnel_enabled { ", tunneled" } else { "" }))
                .collect();
            
            vec![
                ("Service".to_string(), self.service_name.clone()),
                ("Dependencies".to_string(), deps.join(", ")),
            ]
        }
    }
    
    fn title(&self) -> Option<String> {
        Some(format!("Dependencies for '{}'", self.service_name))
    }
}