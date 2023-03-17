use serde::{Deserialize, Serialize};

use strum::{Display, EnumIter};
#[cfg(feature = "winapi")]
use windows::Win32::System::Threading::{PROCESS_CREATION_FLAGS, REALTIME_PRIORITY_CLASS, HIGH_PRIORITY_CLASS, ABOVE_NORMAL_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, IDLE_PRIORITY_CLASS};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessRule {
    pub pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<ProcessPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_affinity: Option<Vec<usize>>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Display, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum ProcessPriority {
    Realtime,
    High,
    AboveNormal,
    Normal,
    BelowNormal,
    Low
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ProcessRuleSet {
    pub rules: Vec<ProcessRule>
}

impl Default for ProcessRule {
    fn default() -> Self {
        Self {
            pattern: "\\w*".into(),
            priority: None,
            core_affinity: None
        }
    }
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