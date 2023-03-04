use regex::Regex;
use serde::Deserialize;

#[cfg(feature = "winapi")]
use windows::Win32::System::Threading::{PROCESS_CREATION_FLAGS, REALTIME_PRIORITY_CLASS, HIGH_PRIORITY_CLASS, ABOVE_NORMAL_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, IDLE_PRIORITY_CLASS};

#[derive(Deserialize, Debug, Clone)]
pub struct ProcessRule {
    #[serde(with = "serde_regex")]
    pub pattern: Regex,
    pub priority: Option<ProcessPriority>,
    pub core_affinity: Option<Vec<usize>>
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ProcessPriority {
    Realtime,
    High,
    AboveNormal,
    Normal,
    BelowNormal,
    Low
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ProcessRuleSet {
    pub rules: Vec<ProcessRule>
}

#[cfg(feature = "winapi")]
impl From<ProcessPriority> for PROCESS_CREATION_FLAGS {
    fn from(value: ProcessPriority) -> Self {
        match value {
            ProcessPriority::Realtime => REALTIME_PRIORITY_CLASS,
            ProcessPriority::High => HIGH_PRIORITY_CLASS,
            ProcessPriority::AboveNormal => ABOVE_NORMAL_PRIORITY_CLASS,
            ProcessPriority::Normal => NORMAL_PRIORITY_CLASS,
            ProcessPriority::BelowNormal => BELOW_NORMAL_PRIORITY_CLASS,
            ProcessPriority::Low => IDLE_PRIORITY_CLASS,
        }
    }
}