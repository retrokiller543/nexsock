use std::io::{self, Write};

use nexsock_protocol::commands::{
    service_status::ServiceStatus,
    list_services::ListServicesResponse,
    config::ServiceConfigPayload,
    dependency::ListDependenciesResponse,
    git::{GitLogResponse, GitListBranchesResponse, RepoStatus},
    error::ErrorPayload,
};

use super::{CliDisplay, DisplayOptions, colors, utils};

impl CliDisplay for ServiceStatus {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        let status_colored = utils::colorize(
            &format!("{:?}", self.state),
            utils::status_color(&format!("{:?}", self.state)),
            options,
        );

        if !options.no_headers {
            writeln!(writer, "{}", utils::colorize("Service Details", colors::BOLD, options))?;
            if options.color {
                utils::write_table_separator(writer, &[50], options)?;
            }
        }

        writeln!(writer, "ID: {}", self.id)?;
        writeln!(writer, "Name: {}", self.name)?;
        writeln!(writer, "Status: {}", status_colored)?;
        writeln!(writer, "Port: {}", self.port)?;
        writeln!(writer, "Repository URL: {}", self.repo_url)?;
        writeln!(writer, "Repository Path: {}", self.repo_path)?;

        if let Some(ref branch) = self.git_branch {
            writeln!(writer, "Git Branch: {}", branch)?;
        }
        if let Some(ref commit) = self.git_commit_hash {
            writeln!(writer, "Git Commit: {}", utils::format_commit_hash(commit, true))?;
        }

        if let Some(ref config) = self.config {
            writeln!(writer)?;
            writeln!(writer, "{}", utils::colorize("Configuration", colors::BOLD, options))?;
            if let Some(ref filename) = config.filename {
                writeln!(writer, "Config File: {}", filename)?;
            }
            if let Some(ref format) = config.format {
                writeln!(writer, "Config Format: {:?}", format)?;
            }
            if let Some(ref run_command) = config.run_command {
                writeln!(writer, "Run Command: {}", run_command)?;
            }
        }

        if !self.dependencies.is_empty() {
            writeln!(writer)?;
            writeln!(writer, "{}", utils::colorize("Dependencies", colors::BOLD, options))?;
            for dep in &self.dependencies {
                let dep_status = utils::colorize(
                    &format!("{:?}", dep.state),
                    utils::status_color(&format!("{:?}", dep.state)),
                    options,
                );
                let tunnel_status = if dep.tunnel_enabled { "tunneled" } else { "direct" };
                writeln!(writer, "  {} ({}) - {}", dep.name, tunnel_status, dep_status)?;
            }
        }

        Ok(())
    }

    fn display_plain(&self, _options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{} ({:?}) - Port {}", self.name, self.state, self.port)
    }
}

impl CliDisplay for ListServicesResponse {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        if self.services.is_empty() {
            writeln!(writer, "No services found")?;
            return Ok(());
        }

        let id_width = self.services.iter().map(|s| s.id.to_string().len()).max().unwrap_or(2).max(2);
        let name_width = self.services.iter().map(|s| s.name.len()).max().unwrap_or(4).max(4);
        let status_width = 8; // Max length of status names
        let port_width = 5;
        let deps_width = 4;

        if !options.no_headers {
            let headers = ["ID", "NAME", "STATUS", "PORT", "DEPS"];
            let widths = [id_width, name_width, status_width, port_width, deps_width];
            
            utils::write_table_row(writer, &headers, &widths, options)?;
            if options.color {
                utils::write_table_separator(writer, &widths, options)?;
            }
        }

        for service in &self.services {
            let status_colored = utils::colorize(
                &format!("{:?}", service.state),
                utils::status_color(&format!("{:?}", service.state)),
                options,
            );
            let deps_indicator = if service.has_dependencies { "Yes" } else { "No" };

            let columns = [
                &service.id.to_string(),
                &service.name,
                &status_colored,
                &service.port.to_string(),
                deps_indicator,
            ];
            let widths = [id_width, name_width, status_width, port_width, deps_width];

            utils::write_table_row(writer, &columns, &widths, options)?;
        }

        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        for service in &self.services {
            writeln!(writer, "{}", service.name)?;
        }
        Ok(())
    }
}

impl CliDisplay for ServiceConfigPayload {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        if !options.no_headers {
            writeln!(writer, "{}", utils::colorize("Service Configuration", colors::BOLD, options))?;
            if options.color {
                utils::write_table_separator(writer, &[50], options)?;
            }
        }

        writeln!(writer, "Service: {}", self.service)?;
        writeln!(writer, "Config File: {}", self.filename)?;
        writeln!(writer, "Format: {:?}", self.format)?;
        writeln!(writer, "Run Command: {}", self.run_command)?;

        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "{}: {} ({})", self.service, self.run_command, self.filename)
    }
}

impl CliDisplay for ListDependenciesResponse {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        if self.dependencies.is_empty() {
            writeln!(writer, "No dependencies found for service '{}'", self.service_name)?;
            return Ok(());
        }

        if !options.no_headers {
            writeln!(writer, "{} Dependencies for '{}'", 
                utils::colorize("Service", colors::BOLD, options), 
                self.service_name)?;
            if options.color {
                utils::write_table_separator(writer, &[50], options)?;
            }
        }

        let name_width = self.dependencies.iter().map(|d| d.name.len()).max().unwrap_or(4).max(4);
        let status_width = 8;
        let tunnel_width = 7;

        if !options.no_headers {
            let headers = ["NAME", "STATUS", "TUNNEL"];
            let widths = [name_width, status_width, tunnel_width];
            
            utils::write_table_row(writer, &headers, &widths, options)?;
            if options.color {
                utils::write_table_separator(writer, &widths, options)?;
            }
        }

        for dep in &self.dependencies {
            let status_colored = utils::colorize(
                &format!("{:?}", dep.state),
                utils::status_color(&format!("{:?}", dep.state)),
                options,
            );
            let tunnel_status = if dep.tunnel_enabled { "Yes" } else { "No" };

            let columns = [&dep.name, &status_colored, tunnel_status];
            let widths = [name_width, status_width, tunnel_width];

            utils::write_table_row(writer, &columns, &widths, options)?;
        }

        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        for dep in &self.dependencies {
            writeln!(writer, "{}", dep.name)?;
        }
        Ok(())
    }
}

impl CliDisplay for GitLogResponse {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        if self.commits.is_empty() {
            writeln!(writer, "No commits found")?;
            return Ok(());
        }

        for (i, commit) in self.commits.iter().enumerate() {
            if i > 0 {
                writeln!(writer)?;
            }

            let hash_colored = utils::colorize(&commit.short_hash, colors::YELLOW, options);
            let author_colored = utils::colorize(&commit.author_name, colors::CYAN, options);
            let date = utils::format_timestamp(&commit.timestamp);

            writeln!(writer, "{} {} {}", hash_colored, author_colored, date)?;
            
            if options.verbose {
                writeln!(writer, "  Email: {}", commit.author_email)?;
                writeln!(writer, "  Full hash: {}", commit.hash)?;
                for line in commit.full_message.lines() {
                    writeln!(writer, "  {}", line)?;
                }
            } else {
                writeln!(writer, "  {}", commit.message)?;
            }
        }

        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        for commit in &self.commits {
            writeln!(writer, "{} {}", commit.short_hash, commit.message)?;
        }
        Ok(())
    }
}

impl CliDisplay for GitListBranchesResponse {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        if self.branches.is_empty() {
            writeln!(writer, "No branches found")?;
            return Ok(());
        }

        if !options.no_headers {
            writeln!(writer, "{}", utils::colorize("Branches", colors::BOLD, options))?;
            if options.color {
                utils::write_table_separator(writer, &[30], options)?;
            }
        }

        for branch in &self.branches {
            writeln!(writer, "{}", branch)?;
        }

        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        for branch in &self.branches {
            writeln!(writer, "{}", branch)?;
        }
        Ok(())
    }
}

impl CliDisplay for RepoStatus {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        if !options.no_headers {
            writeln!(writer, "{}", utils::colorize("Repository Status", colors::BOLD, options))?;
            if options.color {
                utils::write_table_separator(writer, &[50], options)?;
            }
        }

        writeln!(writer, "Remote URL: {}", self.remote_url)?;
        writeln!(writer, "Current Commit: {}", utils::format_commit_hash(&self.current_commit, true))?;
        
        if let Some(ref branch) = self.current_branch {
            writeln!(writer, "Current Branch: {}", branch)?;
        }

        let status_text = if self.is_dirty { "Modified" } else { "Clean" };
        let status_color = if self.is_dirty { colors::YELLOW } else { colors::GREEN };
        writeln!(writer, "Working Directory: {}", utils::colorize(status_text, status_color, options))?;

        if let (Some(ahead), Some(behind)) = (self.ahead_count, self.behind_count) {
            if ahead > 0 || behind > 0 {
                writeln!(writer, "Sync Status: {} ahead, {} behind", ahead, behind)?;
            } else {
                writeln!(writer, "Sync Status: {}", utils::colorize("Up to date", colors::GREEN, options))?;
            }
        }

        if options.verbose && !self.branches.is_empty() {
            writeln!(writer)?;
            writeln!(writer, "{}", utils::colorize("Available Branches", colors::BOLD, options))?;
            for branch in &self.branches {
                writeln!(writer, "  {}", branch)?;
            }
        }

        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        let status = if self.is_dirty { "modified" } else { "clean" };
        writeln!(writer, "{} ({})", self.current_branch.as_deref().unwrap_or("detached"), status)
    }
}

impl CliDisplay for ErrorPayload {
    fn display_table(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        let error_colored = utils::colorize("Error", colors::BRIGHT_RED, options);
        writeln!(writer, "{}: {}", error_colored, self.message)?;
        
        if options.verbose {
            if let Some(ref details) = self.details {
                writeln!(writer, "Details: {}", details)?;
            }
            writeln!(writer, "Error Code: {}", self.code)?;
        }
        
        Ok(())
    }

    fn display_plain(&self, options: &DisplayOptions, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "Error: {}", self.message)
    }
}