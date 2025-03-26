#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VmafLogLevel {
    None = rust_vmaf_sys::VmafLogLevel_VMAF_LOG_LEVEL_NONE,
    Error = rust_vmaf_sys::VmafLogLevel_VMAF_LOG_LEVEL_ERROR,
    Warning = rust_vmaf_sys::VmafLogLevel_VMAF_LOG_LEVEL_WARNING,
    Info = rust_vmaf_sys::VmafLogLevel_VMAF_LOG_LEVEL_INFO,
    Debug = rust_vmaf_sys::VmafLogLevel_VMAF_LOG_LEVEL_DEBUG,
}

impl From<VmafLogLevel> for u32 {
    fn from(level: VmafLogLevel) -> Self {
        level as u32
    }
}

impl Default for VmafLogLevel {
    fn default() -> Self {
        VmafLogLevel::None
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ContextConfig {
    /// How verbose the logger is.
    pub log_level: VmafLogLevel,

    /// How many threads can be used to run feature extractors concurrently. If set to the `None`,
    /// then it means all threads.
    pub threads: Option<u32>,

    /// Compute scores only every N frames. Note that setting an even value for N can lead to
    /// inaccurate results. For more detail, see [#1214](https://github.com/Netflix/vmaf/issues/1214).
    pub subsample: u32,
}
