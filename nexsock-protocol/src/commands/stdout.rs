use crate::commands::manage_service::ServiceRef;
use crate::service_command;

service_command! {
    pub struct GetServiceStdout<ServiceRef, String> = GetServiceStdout
}
