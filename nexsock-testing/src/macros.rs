#[macro_export]
macro_rules! assert_service_eq {
    ($expected:expr, $actual:expr) => {
        assert_eq!($expected.id, $actual.id, "Service IDs should match");
        assert_eq!($expected.name, $actual.name, "Service names should match");
        assert_eq!($expected.port, $actual.port, "Service ports should match");
        assert_eq!(
            $expected.repo_url, $actual.repo_url,
            "Service repo URLs should match"
        );
        assert_eq!(
            $expected.repo_path, $actual.repo_path,
            "Service repo paths should match"
        );
        assert_eq!(
            $expected.status, $actual.status,
            "Service statuses should match"
        );
    };
}

#[macro_export]
macro_rules! assert_service_status {
    ($service:expr, $expected_status:expr) => {
        assert_eq!(
            $service.status, $expected_status,
            "Service '{}' should have status {:?}, but was {:?}",
            $service.name, $expected_status, $service.status
        );
    };
}

#[macro_export]
macro_rules! test_async {
    ($test_name:ident, $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            $crate::setup::init_test_tracing();
            $test_body
        }
    };
}

#[macro_export]
macro_rules! test_with_db {
    ($test_name:ident, $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            $crate::setup::init_test_tracing();
            let db = $crate::database::setup_test_db()
                .await
                .expect("Failed to setup test database");
            $test_body
        }
    };
}

#[macro_export]
macro_rules! test_with_fixtures {
    ($test_name:ident, $fixtures:expr, $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            $crate::setup::init_test_tracing();
            let db = $crate::database::setup_test_db()
                .await
                .expect("Failed to setup test database");
            let fixtures = $crate::fixtures::create_fixtures(&db, $fixtures)
                .await
                .expect("Failed to create fixtures");
            $test_body
        }
    };
}

#[macro_export]
macro_rules! test_integration {
    ($test_name:ident, $test_body:block) => {
        #[cfg(feature = "integration")]
        #[tokio::test]
        async fn $test_name() {
            $crate::setup::init_test_tracing();
            let test_env = $crate::setup::setup_integration_test()
                .await
                .expect("Failed to setup integration test");
            $test_body
        }
    };
}

#[macro_export]
macro_rules! assert_command_success {
    ($result:expr) => {
        match $result {
            Ok(response) => response,
            Err(e) => panic!("Command should have succeeded but failed with: {}", e),
        }
    };
    ($result:expr, $message:expr) => {
        match $result {
            Ok(response) => response,
            Err(e) => panic!("{}: {}", $message, e),
        }
    };
}

#[macro_export]
macro_rules! assert_command_error {
    ($result:expr) => {
        match $result {
            Ok(_) => panic!("Command should have failed but succeeded"),
            Err(e) => e,
        }
    };
    ($result:expr, $expected_error:expr) => {
        match $result {
            Ok(_) => panic!("Command should have failed but succeeded"),
            Err(e) => {
                let error_string = e.to_string();
                assert!(
                    error_string.contains($expected_error),
                    "Expected error to contain '{}', but got: {}",
                    $expected_error,
                    error_string
                );
            }
        }
    };
}

#[macro_export]
macro_rules! timeout_test {
    ($duration:expr, $test_body:expr) => {
        tokio::time::timeout($duration, async { $test_body })
            .await
            .expect("Test timed out")
    };
}

#[macro_export]
macro_rules! mock_service {
    ($name:expr) => {
        $crate::fixtures::ServiceFixture::new($name)
    };
    ($name:expr, port: $port:expr) => {
        $crate::fixtures::ServiceFixture::new($name).with_port($port)
    };
    ($name:expr, status: $status:expr) => {
        $crate::fixtures::ServiceFixture::new($name).with_status($status)
    };
    ($name:expr, port: $port:expr, status: $status:expr) => {
        $crate::fixtures::ServiceFixture::new($name)
            .with_port($port)
            .with_status($status)
    };
}

#[macro_export]
macro_rules! assert_logs_contain {
    ($pattern:expr) => {
        // This would need integration with a log capture system
        // For now, this is a placeholder for future implementation
        std::todo!("Log assertion not yet implemented");
    };
}

#[macro_export]
macro_rules! parallel_test {
    ($test_name:ident, $count:expr, $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            $crate::setup::init_test_tracing();

            let tasks: Vec<_> = (0..$count)
                .map(|i| {
                    tokio::spawn(async move {
                        tracing::info!("Starting parallel test iteration {}", i);
                        $test_body
                        tracing::info!("Completed parallel test iteration {}", i);
                    })
                })
                .collect();

            for task in tasks {
                task.await.expect("Parallel test task failed");
            }
        }
    };
}
