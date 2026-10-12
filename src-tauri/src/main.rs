// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // WebKitGTK's DMABUF renderer crashes on Wayland with NVIDIA drivers
    // ("Error 71 (Protocol error) dispatching to Wayland display").
    // Must run before GTK/WebKit init and before any threads are spawned.
    // An explicit user value is respected (e.g. =0 to re-enable DMABUF).
    #[cfg(target_os = "linux")]
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    pomotroid_lib::run()
}
