#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

pub trait ClipboardManager {
    fn get_clipboard(&self) -> Option<String>;
    fn set_clipboard(&self, payload: &str);
}

// TODO: change windows and linux implementations to avoid initialize() call and use constructor instead

// LINUX IMPLEMENTATION
#[cfg(target_os = "linux")]
pub fn get_manager() -> impl ClipboardManager {
    let manager = linux::LinuxClipboardManager{};
    manager.initialize();
    manager
}

// WINDOWS IMPLEMENTATION
#[cfg(target_os = "windows")]
pub fn get_manager() -> impl ClipboardManager {
    windows::WindowsClipboardManager::new()
}

// MAC IMPLEMENTATION
#[cfg(target_os = "macos")]
pub fn get_manager() -> impl ClipboardManager {
    macos::MacClipboardManager::new()
}