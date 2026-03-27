/** True when running on macOS inside the Tauri desktop app. */
export const isMac = /Macintosh|Mac OS X/.test(navigator.userAgent);

/** True when running on Linux inside the Tauri desktop app. */
export const isLinux = /Linux/.test(navigator.userAgent);
