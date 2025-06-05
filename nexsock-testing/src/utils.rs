use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};

pub async fn wait_for<F, Fut>(mut condition: F, timeout_duration: Duration) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = Instant::now();

    loop {
        if condition().await {
            return Ok(());
        }

        if start.elapsed() > timeout_duration {
            return Err(anyhow::anyhow!(
                "Condition not met within timeout of {:?}",
                timeout_duration
            ));
        }

        sleep(Duration::from_millis(50)).await;
    }
}

pub async fn wait_for_sync<F>(mut condition: F, timeout_duration: Duration) -> Result<()>
where
    F: FnMut() -> bool,
{
    let start = Instant::now();

    loop {
        if condition() {
            return Ok(());
        }

        if start.elapsed() > timeout_duration {
            return Err(anyhow::anyhow!(
                "Condition not met within timeout of {:?}",
                timeout_duration
            ));
        }

        sleep(Duration::from_millis(50)).await;
    }
}

pub async fn retry<F, Fut, T, E>(
    mut operation: F,
    max_attempts: usize,
    delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error = None;

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                tracing::debug!("Attempt {}/{} failed: {}", attempt, max_attempts, error);
                last_error = Some(error);

                if attempt < max_attempts {
                    sleep(delay).await;
                }
            }
        }
    }

    Err(last_error.unwrap())
}

pub async fn eventually<F, Fut, T>(
    mut operation: F,
    timeout_duration: Duration,
    check_interval: Duration,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    timeout(timeout_duration, async {
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    tracing::trace!("Operation failed, retrying: {}", error);
                    sleep(check_interval).await;
                }
            }
        }
    })
    .await
    .map_err(|_| anyhow::anyhow!("Operation timed out after {:?}", timeout_duration))?
}

pub fn generate_test_id() -> String {
    format!("test_{}", uuid::Uuid::new_v4().simple())
}

pub fn generate_test_name(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::new_v4().simple())
}

pub fn is_port_available(port: u16) -> bool {
    use std::net::{SocketAddr, TcpListener};

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    TcpListener::bind(addr).is_ok()
}

pub fn find_available_port() -> Result<u16> {
    use std::net::{SocketAddr, TcpListener};

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let port = listener.local_addr()?.port();
    Ok(port)
}

pub async fn wait_for_port(port: u16, timeout_duration: Duration) -> Result<()> {
    wait_for_sync(|| !is_port_available(port), timeout_duration).await
}

pub async fn wait_for_port_free(port: u16, timeout_duration: Duration) -> Result<()> {
    wait_for_sync(|| is_port_available(port), timeout_duration).await
}

pub async fn with_timeout<T>(
    future: impl std::future::Future<Output = T>,
    timeout_duration: Duration,
) -> Result<T> {
    timeout(timeout_duration, future)
        .await
        .map_err(|_| anyhow::anyhow!("Operation timed out after {:?}", timeout_duration))
}

pub fn create_temp_config_file(content: &str) -> Result<tempfile::NamedTempFile> {
    use std::io::Write;

    let mut file = tempfile::NamedTempFile::new()?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(file)
}

pub fn create_temp_dir_with_files(files: &[(&str, &str)]) -> Result<tempfile::TempDir> {
    let temp_dir = tempfile::tempdir()?;

    for (filename, content) in files {
        let file_path = temp_dir.path().join(filename);
        std::fs::write(file_path, content)?;
    }

    Ok(temp_dir)
}

pub async fn assert_eventually<F, Fut>(
    condition: F,
    timeout_duration: Duration,
    message: &str,
) -> Result<()>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    match wait_for(condition, timeout_duration).await {
        Ok(()) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("Assertion failed: {}", message)),
    }
}

pub async fn measure_time<F, Fut, T>(operation: F) -> (T, Duration)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let start = Instant::now();
    let result = operation().await;
    let duration = start.elapsed();
    (result, duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_for_sync() {
        let mut counter = 0;
        let result = wait_for_sync(
            || {
                counter += 1;
                counter > 3
            },
            Duration::from_secs(1),
        )
        .await;

        assert!(result.is_ok());
        assert!(counter > 3);
    }

    #[tokio::test]
    async fn test_wait_for_timeout() {
        let result = wait_for_sync(|| false, Duration::from_millis(100)).await;

        assert!(result.is_err());
    }

    // TODO: Fix closure lifetime issues
    // #[tokio::test]
    // async fn test_retry() {
    //     let mut attempts = 0;
    //     let result = retry(
    //         || async {
    //             attempts += 1;
    //             if attempts < 3 {
    //                 Err("Not ready")
    //             } else {
    //                 Ok("Success")
    //             }
    //         },
    //         5,
    //         Duration::from_millis(10),
    //     ).await;
    //
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), "Success");
    //     assert_eq!(attempts, 3);
    // }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let result: Result<(), &str> = retry(
            || async { Err("Always fails") },
            3,
            Duration::from_millis(10),
        )
        .await;

        assert!(result.is_err());
    }

    // TODO: Fix closure lifetime issues
    // #[tokio::test]
    // async fn test_eventually() {
    //     let mut counter = 0;
    //     let result = eventually(
    //         || async {
    //             counter += 1;
    //             if counter > 2 {
    //                 Ok("Done")
    //             } else {
    //                 Err(anyhow::anyhow!("Not ready"))
    //             }
    //         },
    //         Duration::from_secs(1),
    //         Duration::from_millis(10),
    //     ).await;
    //
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), "Done");
    // }

    #[test]
    fn test_generate_test_id() {
        let id1 = generate_test_id();
        let id2 = generate_test_id();

        assert!(id1.starts_with("test_"));
        assert!(id2.starts_with("test_"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_test_name() {
        let name1 = generate_test_name("service");
        let name2 = generate_test_name("service");

        assert!(name1.starts_with("service_"));
        assert!(name2.starts_with("service_"));
        assert_ne!(name1, name2);
    }

    #[test]
    fn test_find_available_port() {
        let port = find_available_port().unwrap();
        assert!(port > 0);
        assert!(is_port_available(port));
    }

    #[test]
    fn test_create_temp_config_file() {
        let content = "test=value\nother=123";
        let file = create_temp_config_file(content).unwrap();

        let read_content = std::fs::read_to_string(file.path()).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_create_temp_dir_with_files() {
        let files = &[
            ("config.env", "PORT=8080\nHOST=localhost"),
            ("run.sh", "#!/bin/bash\necho 'starting service'"),
        ];

        let temp_dir = create_temp_dir_with_files(files).unwrap();

        let config_content = std::fs::read_to_string(temp_dir.path().join("config.env")).unwrap();
        assert_eq!(config_content, "PORT=8080\nHOST=localhost");

        let script_content = std::fs::read_to_string(temp_dir.path().join("run.sh")).unwrap();
        assert_eq!(script_content, "#!/bin/bash\necho 'starting service'");
    }

    #[tokio::test]
    async fn test_measure_time() {
        let (result, duration) = measure_time(|| async {
            sleep(Duration::from_millis(100)).await;
            "done"
        })
        .await;

        assert_eq!(result, "done");
        assert!(duration >= Duration::from_millis(90)); // Allow some tolerance
        assert!(duration < Duration::from_millis(200));
    }
}
