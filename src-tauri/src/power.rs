//! Keep-awake / prevent-sleep for Windows.
//! Uses SetThreadExecutionState — a stable, documented Win32 API.
//! No-ops on non-Windows builds.

#[cfg(target_os = "windows")]
mod win {
    use windows::Win32::System::Power::{
        SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
        EXECUTION_STATE,
    };

    pub fn acquire(keep_screen: bool) {
        let mut flags: EXECUTION_STATE = ES_CONTINUOUS | ES_SYSTEM_REQUIRED;
        if keep_screen {
            flags |= ES_DISPLAY_REQUIRED;
        }
        unsafe {
            SetThreadExecutionState(flags);
        }
        log::debug!("[power] wakelock acquired keep_screen={keep_screen}");
    }

    pub fn release() {
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS);
        }
        log::debug!("[power] wakelock released");
    }
}

#[allow(unused_variables)]
pub fn acquire_wakelock(keep_screen: bool) {
    #[cfg(target_os = "windows")]
    win::acquire(keep_screen);
}

pub fn release_wakelock() {
    #[cfg(target_os = "windows")]
    win::release();
}