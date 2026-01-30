// Prevents console window in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod connection;
#[cfg(target_os = "macos")]
mod mac_ui;
#[cfg(not(target_os = "macos"))]
mod win_ui;

use tao::{
    event_loop::EventLoopBuilder,
    window::{WindowBuilder, Icon},
    dpi::LogicalSize,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum UserEvent { 
    UpdateStatus(bool), 
    UpdateProgress(u32),
    GoBack,
    GoForward,
    Reload,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-features=IsolateOrigins,site-per-process --disable-blink-features=AutomationControlled --no-sandbox --disable-setuid-sandbox --disable-infobars --disable-dev-shm-usage --ignore-certificate-errors --allow-running-insecure-content --test-type --ignore-urlfetcher-cert-requests");

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    
    let window = WindowBuilder::new()
        .with_title("Matchapro GC Desktop")
        .with_maximized(true)
        .with_decorations(false)
        .with_window_icon(load_icon())
        .with_inner_size(LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(LogicalSize::new(900.0, 600.0))
        .build(&event_loop)?;
    let window = Arc::new(window);

    #[cfg(target_os = "macos")]
    {
        mac_ui::run(window, event_loop, proxy).await
    }

    #[cfg(not(target_os = "macos"))]
    {
        win_ui::run(window, event_loop, proxy).await
    }
}

fn load_icon() -> Option<Icon> {
    let icon_bytes = include_bytes!("../logo-app.png");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Icon::from_rgba(rgba.into_raw(), width, height).ok()
    } else { None }
}
