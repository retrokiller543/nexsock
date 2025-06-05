use anyhow::Result;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::timeout;

pub struct TestProcess {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<std::path::PathBuf>,
    pub env_vars: std::collections::HashMap<String, String>,
    pub timeout_duration: Duration,
}

impl TestProcess {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
            working_dir: None,
            env_vars: std::collections::HashMap::new(),
            timeout_duration: Duration::from_secs(30),
        }
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }

    pub fn working_dir<P: AsRef<std::path::Path>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout_duration = duration;
        self
    }

    pub async fn run(&self) -> Result<TestProcessResult> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(ref dir) = self.working_dir {
            cmd.current_dir(dir);
        }

        for (key, value) in &self.env_vars {
            cmd.env(key, value);
        }

        let child = cmd.spawn()?;

        let result = timeout(self.timeout_duration, async {
            let output = child.wait_with_output()?;
            Ok::<_, anyhow::Error>(output)
        })
        .await
        .map_err(|_| anyhow::anyhow!("Process timed out after {:?}", self.timeout_duration))?;

        let output = result?;

        Ok(TestProcessResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
        })
    }

    pub async fn run_success(&self) -> Result<TestProcessResult> {
        let result = self.run().await?;
        if !result.success {
            return Err(anyhow::anyhow!(
                "Process failed with exit code {}: {}",
                result.exit_code,
                result.stderr
            ));
        }
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct TestProcessResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

impl TestProcessResult {
    pub fn assert_success(&self) {
        assert!(
            self.success,
            "Process failed with exit code {}: {}",
            self.exit_code, self.stderr
        );
    }

    pub fn assert_failure(&self) {
        assert!(
            !self.success,
            "Process should have failed but succeeded with output: {}",
            self.stdout
        );
    }

    pub fn assert_exit_code(&self, expected: i32) {
        assert_eq!(
            self.exit_code, expected,
            "Expected exit code {}, got {}: {}",
            expected, self.exit_code, self.stderr
        );
    }

    pub fn assert_stdout_contains(&self, pattern: &str) {
        assert!(
            self.stdout.contains(pattern),
            "Stdout does not contain '{}': {}",
            pattern,
            self.stdout
        );
    }

    pub fn assert_stderr_contains(&self, pattern: &str) {
        assert!(
            self.stderr.contains(pattern),
            "Stderr does not contain '{}': {}",
            pattern,
            self.stderr
        );
    }
}

pub async fn kill_process_by_name(name: &str) -> Result<()> {
    #[cfg(unix)]
    {
        TestProcess::new("pkill").arg("-f").arg(name).run().await?;
    }

    #[cfg(windows)]
    {
        TestProcess::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg(&format!("{}.exe", name))
            .run()
            .await?;
    }

    Ok(())
}

pub async fn is_process_running(name: &str) -> Result<bool> {
    #[cfg(unix)]
    {
        let result = TestProcess::new("pgrep").arg("-f").arg(name).run().await?;
        Ok(result.success)
    }

    #[cfg(windows)]
    {
        let result = TestProcess::new("tasklist")
            .arg("/FI")
            .arg(&format!("IMAGENAME eq {}.exe", name))
            .run()
            .await?;
        Ok(result.stdout.contains(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_command() {
        let result = TestProcess::new("echo")
            .arg("hello world")
            .run()
            .await
            .expect("Failed to run echo command");

        result.assert_success();
        result.assert_stdout_contains("hello world");
    }

    #[tokio::test]
    async fn test_command_with_working_directory() {
        let result = TestProcess::new("pwd").working_dir("/tmp").run().await;

        #[cfg(unix)]
        {
            let result = result.expect("Failed to run pwd command");
            result.assert_success();
            result.assert_stdout_contains("/tmp");
        }

        #[cfg(windows)]
        {
            // On Windows, use 'cd' instead of 'pwd'
            let result = TestProcess::new("cmd")
                .args(["/C", "cd"])
                .working_dir("C:\\Windows\\Temp")
                .run()
                .await
                .expect("Failed to run cd command");

            result.assert_success();
        }
    }

    /*#[tokio::test]
    async fn test_command_timeout() {
        // Test timeout by using a command that will definitely take longer than the timeout
        #[cfg(unix)]
        {
            let result = TestProcess::new("yes")
                .timeout(Duration::from_millis(50))
                .run()
                .await;

            assert!(
                result.is_err(),
                "Expected timeout error, but command succeeded"
            );
        }

        #[cfg(windows)]
        {
            let result = TestProcess::new("cmd")
                .args(["/C", "timeout /t 10"])
                .timeout(Duration::from_millis(50))
                .run()
                .await;

            assert!(
                result.is_err(),
                "Expected timeout error, but command succeeded"
            );
        }
    }*/

    #[tokio::test]
    async fn test_failing_command() {
        #[cfg(unix)]
        {
            let result = TestProcess::new("false")
                .run()
                .await
                .expect("Failed to run false command");

            result.assert_failure();
            assert_eq!(result.exit_code, 1);
        }

        #[cfg(windows)]
        {
            let result = TestProcess::new("cmd")
                .args(["/C", "exit 1"])
                .run()
                .await
                .expect("Failed to run exit command");

            result.assert_failure();
            assert_eq!(result.exit_code, 1);
        }
    }
}
