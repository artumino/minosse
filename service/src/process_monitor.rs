use common::*;

use serde::Deserialize;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::StreamExt;
use windows::Win32::System::Threading::{
    OpenProcess, SetPriorityClass, SetProcessAffinityMask, PROCESS_SET_INFORMATION,
};
use wmi::*;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_ProcessStartTrace")]
#[serde(rename_all = "PascalCase")]
struct ProcessStartTrace {
    process_id: u32,
    process_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Process")]
#[serde(rename_all = "PascalCase")]
struct WinProcess {
    process_id: u32,
    name: String,
}

struct ProcessInfo {
    process_id: u32,
    process_name: String,
}

impl From<ProcessStartTrace> for ProcessInfo {
    fn from(value: ProcessStartTrace) -> Self {
        Self {
            process_id: value.process_id,
            process_name: value.process_name,
        }
    }
}

impl From<WinProcess> for ProcessInfo {
    fn from(value: WinProcess) -> Self {
        Self {
            process_id: value.process_id,
            process_name: value.name,
        }
    }
}

pub trait RuleApplier<RuleInfo> {
    fn apply(&self, process: &RuleInfo) -> anyhow::Result<()>;
}

impl RuleApplier<ProcessInfo> for ProcessRuleSet {
    fn apply(&self, process: &ProcessInfo) -> anyhow::Result<()> {
        self.rules.iter().for_each(|rule| {
            if let Err(error) = rule.apply(process) {
                println!(
                    "Error applying rule: {:?} for process {:?}",
                    error, process.process_name
                );
            }
        });

        Ok(())
    }
}

impl RuleApplier<ProcessInfo> for ProcessRule {
    fn apply(&self, process: &ProcessInfo) -> anyhow::Result<()> {
        if self.pattern.is_match(&process.process_name) {
            let process_handle =
                unsafe { OpenProcess(PROCESS_SET_INFORMATION, false, process.process_id) }?;

            if let Some(ref core_affinity) = self.core_affinity {
                let affinity_mask = core_affinity
                    .iter()
                    .fold(0, |mask, core| mask | (1usize << core));
                let result = unsafe { SetProcessAffinityMask(process_handle, affinity_mask) };
                if !result.as_bool() {
                    anyhow::bail!(
                        "Failed to set core affinity for process {:?}",
                        process.process_name
                    );
                }
            }

            if let Some(priority) = &self.priority {
                let result = unsafe { SetPriorityClass(process_handle, (*priority).into()) };
                if !result.as_bool() {
                    anyhow::bail!(
                        "Failed to set priority for process {:?}",
                        process.process_name
                    );
                }
            }

            println!(
                "Applied rule {:?} to process {:?}",
                self.pattern.as_str(),
                process.process_name
            )
        }
        Ok(())
    }
}

async fn monitor_new_processes(
    rule_set: &ProcessRuleSet,
    wmi_con: &WMIConnection,
) -> anyhow::Result<()> {
    let mut process_start_stream = wmi_con.async_notification::<ProcessStartTrace>()?;
    while let Some(Ok(event)) = process_start_stream.next().await {
        let process_info: ProcessInfo = event.into();
        rule_set.apply(&process_info).unwrap();
    }

    Ok(())
}

pub(crate) async fn rule_applier(
    rule_file_path: &str,
    shutdown_recv: &mut UnboundedReceiver<()>,
) -> anyhow::Result<()> {
    let wmi_con = WMIConnection::new(COMLibrary::new()?)?;
    let rule_set: ProcessRuleSet = std::fs::File::open(rule_file_path)
        .map(|file| serde_json::from_reader(file).unwrap_or(ProcessRuleSet { rules: vec![] }))
        .unwrap_or(ProcessRuleSet { rules: vec![] });

    // Apply rules to all running processes
    let running_process: Vec<WinProcess> = wmi_con.async_query().await?;
    running_process.into_iter().for_each(|process| {
        let process_info: ProcessInfo = process.into();
        rule_set.apply(&process_info).unwrap();
    });

    tokio::select! {
        // Apply rules to new processes
        output = monitor_new_processes(&rule_set, &wmi_con) => output,
        // Or wait for shutdown signal
        _ = shutdown_recv.recv() => {
            println!("Shutting down process monitor");
            Ok(())
        }
    }
}
