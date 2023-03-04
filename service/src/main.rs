mod process_monitor;
#[macro_use]
extern crate windows_service;

use crate::process_monitor::rule_applier;
use tokio::runtime::Runtime;
use std::ffi::OsString;
use windows_service::service_dispatcher;

const SERVICE_NAME: &str = "Minosse - Process Monitor Service";
define_windows_service!(ffi_service_main, service_main);

#[cfg(windows)]
fn service_main(_: Vec<OsString>) {
    use std::time::Duration;

    use tokio::sync::mpsc;
    use windows_service::{service_control_handler::{ServiceControlHandlerResult, self}, service::{ServiceControl, ServiceStatus, ServiceControlAccept, ServiceExitCode, ServiceType, ServiceState}};

    let rt  = Runtime::new().unwrap();
    let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop => {
                shutdown_send.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler).unwrap();
    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }).unwrap();
    
    let args = std::env::args().collect::<Vec<_>>();
    let rules_path = args.get(2).map(|s| s.as_str()).unwrap_or("rules.json");
    
    let error_code = if rt.block_on(rule_applier(rules_path, &mut shutdown_recv)).is_err() {
        1
    } else {
        0
    };

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(error_code),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }).unwrap();

}

#[cfg(windows)]
fn main() -> Result<(), windows_service::Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let command = args.get(1);
    if let Some(command) = command {
        match command.as_str() {
            "install" => {
                install_service(args.get(2).map(|s| s.as_str()))?;
                println!("Service installed");
                return Ok(());
            }
            "uninstall" => {
                uninstall_service()?;
                println!("Service uninstalled");
                return Ok(());
            },
            "run" => {},
            _ => {
                println!("Unknown command");
                return Ok(());
            }
        }
    }

    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

#[cfg(windows)]
fn install_service(rules_path: Option<&str>) -> windows_service::Result<()> {
    use std::path::Path;

    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_binary_path = ::std::env::current_exe()
        .unwrap();

    let rule_file_path = rules_path.unwrap_or("rules.json");

    let service_info = ServiceInfo {
        name: SERVICE_NAME.into(),
        display_name: SERVICE_NAME.into(),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec!("run".into(), Path::new(rule_file_path).canonicalize().unwrap().into()),
        dependencies: vec![],
        account_name: None, // run as System
        account_password: None,
        
    };
    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
    service.set_description("Monitoring process that sets process affinities and priorities based on user-defined rules")?;
    Ok(())
}


#[cfg(windows)]
fn uninstall_service() -> windows_service::Result<()> {
    use windows_service::{
        service_manager::{ServiceManager, ServiceManagerAccess}, service::ServiceAccess,
    };

    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service = service_manager.open_service(SERVICE_NAME, ServiceAccess::DELETE)?;
    service.delete()?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This program only works on Windows");
}
