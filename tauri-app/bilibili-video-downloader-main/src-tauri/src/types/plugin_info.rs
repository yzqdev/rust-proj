use std::path::PathBuf;

use bilibili_video_downloader_plugin_api::v1::{
    HookPointV1, PluginDescriptorV1, PluginFailurePolicy,
};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::plugin::plugin_types::PluginRuntime;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum PluginHookPoint {
    #[default]
    BeforeVideoProcess,
    AfterPrepare,
    OnCompleted,
}

impl From<HookPointV1> for PluginHookPoint {
    fn from(value: HookPointV1) -> Self {
        match value {
            HookPointV1::BeforeVideoProcess => Self::BeforeVideoProcess,
            HookPointV1::AfterPrepare => Self::AfterPrepare,
            HookPointV1::OnCompleted => Self::OnCompleted,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum PluginFailurePolicyInfo {
    #[default]
    FailOpen,
    FailClosed,
}

impl From<PluginFailurePolicy> for PluginFailurePolicyInfo {
    fn from(value: PluginFailurePolicy) -> Self {
        match value {
            PluginFailurePolicy::FailOpen => Self::FailOpen,
            PluginFailurePolicy::FailClosed => Self::FailClosed,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct PluginDescriptorInfo {
    pub sdk_api_version: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub hooks: Vec<PluginHookPoint>,
    pub failure_policy: PluginFailurePolicyInfo,
    pub description: String,
}

impl PluginDescriptorInfo {
    pub fn from_descriptor(descriptor: &PluginDescriptorV1) -> Self {
        Self {
            sdk_api_version: descriptor.sdk_api_version,
            id: descriptor.id.clone(),
            name: descriptor.name.clone(),
            version: descriptor.version.clone(),
            hooks: descriptor
                .hooks
                .iter()
                .copied()
                .map(PluginHookPoint::from)
                .collect(),
            failure_policy: descriptor.failure_policy.into(),
            description: descriptor.description.clone(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct PluginMetadata {
    pub path: PathBuf,
    pub enabled: bool,
    pub priority: i32,
    pub descriptor: PluginDescriptorInfo,
}

impl PluginMetadata {
    pub fn from_plugin_runtime(runtime: &PluginRuntime) -> Self {
        Self {
            path: runtime.plugin_path.clone(),
            enabled: runtime.enabled,
            priority: runtime.priority,
            descriptor: PluginDescriptorInfo::from_descriptor(&runtime.descriptor),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum PluginRuntimeStatus {
    #[default]
    Unknown,
    Loaded,
    Disabled,
    LoadFailed,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct PluginInfo {
    pub path: PathBuf,
    pub enabled: bool,
    pub priority: i32,
    pub descriptor: PluginDescriptorInfo,
    pub runtime_status: PluginRuntimeStatus,
}

impl PluginInfo {
    pub fn from_metadata(metadata: PluginMetadata, runtime_status: PluginRuntimeStatus) -> Self {
        Self {
            path: metadata.path,
            enabled: metadata.enabled,
            priority: metadata.priority,
            descriptor: metadata.descriptor,
            runtime_status,
        }
    }

    pub fn into_metadata(self) -> PluginMetadata {
        PluginMetadata {
            path: self.path,
            enabled: self.enabled,
            priority: self.priority,
            descriptor: self.descriptor,
        }
    }
}
