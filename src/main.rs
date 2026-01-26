// Prevents console window in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod connection;

use tao::{
    event::{Event, WindowEvent},
    window::{WindowBuilder, Icon},
    dpi::LogicalSize,
};


use wry::{WebViewBuilder, WebContext, WebView};
use std::sync::Arc;
use std::time::Duration;
use base64::prelude::*;

const SE_BLACK_BYTES: &[u8] = include_bytes!("../se-black.png");

const TARGET_URL: &str = "https://matchapro.web.bps.go.id/login";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 12; M2010J19CG Build/SKQ1.211202.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/120.0.0.0 Mobile Safari/537.36";
const FOOTER_HEIGHT: u32 = 40;
const HEADER_HEIGHT: u32 = 48;




// --- EMBEDDED ASSETS ---
const CSS_STYLES: &str = include_str!("styles.css");

const HEADER_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
<style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body { 
        height: 48px; width: 100%; margin: 0; padding: 0;
        font-family: -apple-system, system-ui, sans-serif; 
        background: #ffffff; border-bottom: 1px solid #eee; user-select: none; overflow: hidden;
    }
    .header-container { 
        width: 100%; height: 48px; position: relative; padding: 0 15px; overflow: hidden;
    }
    
    .drag-area { position: absolute; top: 0; left: 0; width: 100%; height: 48px; z-index: 1; }
    
    .title { 
        position: absolute; top: 50%; left: 15px; transform: translateY(-50%);
        z-index: 2; pointer-events: none; white-space: nowrap;
        line-height: normal; /* Reset line-height to avoid offset */
    }
    .title img { vertical-align: middle; }
    .title span { font-size: 13px; font-weight: 600; color: #333; margin-left: 10px; vertical-align: middle; }
    
    .browser-controls { 
        position: absolute; top: 0; left: 220px; height: 48px; z-index: 10;
    }
    .nav-btn { 
        float: left; width: 32px; height: 32px; margin-top: 8px; margin-right: 4px;
        text-align: center; line-height: 32px;
        border-radius: 6px; cursor: pointer; transition: 0.2s; color: #555;
    }
    .nav-btn:hover { background: #f0f0f0; color: #000; }
    .nav-btn svg { width: 18px; height: 18px; fill: currentColor; vertical-align: middle; margin-top: -2px; }

    .controls { 
        position: absolute; right: 0; top: 0; height: 48px; z-index: 10;
    }
    .btn { 
        float: left; width: 48px; height: 100%; text-align: center; line-height: 48px;
        cursor: pointer; transition: background 0.2s; color: #444; 
    }
    .btn:hover { background: rgba(0,0,0,0.05); }
    .btn#close:hover { background: #e81123; color: white; }
    .btn svg { width: 10px; height: 10px; vertical-align: middle; }
    
    #progress-container { position: absolute; bottom: 0; left: 0; width: 100%; height: 2px; background: transparent; z-index: 20; }
    #progress-bar { width: 0%; height: 100%; background: #007bff; transition: width 0.3s, opacity 0.5s; }
</style>
</head>
<body onmousedown="if(event.button === 0 && !event.target.closest('.btn') && !event.target.closest('.nav-btn')) window.ipc.postMessage('drag')">
    <div class="header-container">
        <div class="drag-area"></div>
        <div class="title">
            <!-- LOGO_PLACEHOLDER -->
            <span>Matchapro GC Desktop</span>
        </div>
        <div class="browser-controls">
            <div class="nav-btn" onclick="window.ipc.postMessage('back')" title="Back">
                <svg viewBox="0 0 24 24"><path d="M20,11V13H8L13.5,18.5L12.08,19.92L4.16,12L12.08,4.08L13.5,5.5L8,11H20Z" /></svg>
            </div>
            <div class="nav-btn" onclick="window.ipc.postMessage('forward')" title="Forward">
                <svg viewBox="0 0 24 24"><path d="M4,11V13H16L10.5,18.5L11.92,19.92L19.84,12L11.92,4.08L10.5,5.5L16,11H4Z" /></svg>
            </div>
            <div class="nav-btn" onclick="window.ipc.postMessage('refresh')" title="Refresh">
                <svg viewBox="0 0 24 24"><path d="M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z" /></svg>
            </div>
        </div>
        <div class="controls">
            <div class="btn" onclick="window.ipc.postMessage('minimize')"><svg viewBox="0 0 10 1"><rect width="10" height="1" /></svg></div>
            <div class="btn" id="maximize-btn" onclick="window.ipc.postMessage('maximize')"><svg viewBox="0 0 10 10"><path d="M0,0v10h10V0H0z M9,9H1V1h8V9z" /></svg></div>
            <div class="btn" id="close" onclick="window.ipc.postMessage('close')"><svg viewBox="0 0 10 10"><path d="M0,0l10,10M10,0L0,10" stroke="currentColor" stroke-width="1.2"/></svg></div>
        </div>
        <div id="progress-container"><div id="progress-bar"></div></div>
    </div>
    <script>
        window.updateProgress = (p) => {
            const bar = document.getElementById('progress-bar');
            if (p >= 100) {
                bar.style.width = '100%';
                setTimeout(() => { bar.style.opacity = '0'; setTimeout(() => { bar.style.width = '0%'; bar.style.opacity = '1'; }, 500); }, 500);
            } else {
                bar.style.opacity = '1';
                bar.style.width = p + '%';
            }
        };
    </script>
</body>
</html>
"#;








const TOOLBAR_JS: &str = r#"

// Toolbar injection logic
(function() {
    if (window.toolbarInjected) return;
    window.toolbarInjected = true;
    
    const style = document.createElement('style');
    style.textContent = `
        #matchapro-desktop-toolbar {
            position: fixed;
            top: 20px;
            right: 20px;
            z-index: 1000000;
            display: flex;
            gap: 10px;
            background: rgba(255, 255, 255, 0.9);
            padding: 8px 12px;
            border-radius: 30px;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
            backdrop-filter: blur(5px);
            border: 1px solid rgba(0, 0, 0, 0.1);
            user-select: none;
            transition: opacity 0.3s;
        }
        .toolbar-btn {
            width: 32px;
            height: 32px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 50%;
            cursor: pointer;
            color: #555;
            transition: all 0.2s;
        }
        .toolbar-btn:hover { background: #eee; transform: translateY(-1px); color: #000; }
        .toolbar-btn svg { width: 18px; height: 18px; fill: currentColor; }
    `;
    document.head.appendChild(style);

    const toolbar = document.createElement('div');
    toolbar.id = 'matchapro-desktop-toolbar';
    toolbar.innerHTML = `
        <div class="toolbar-btn" onclick="window.history.back()" title="Back">
            <svg viewBox="0 0 24 24"><path d="M20,11V13H8L13.5,18.5L12.08,19.92L4.16,12L12.08,4.08L13.5,5.5L8,11H20Z" /></svg>
        </div>
        <div class="toolbar-btn" onclick="window.history.forward()" title="Forward">
            <svg viewBox="0 0 24 24"><path d="M4,11V13H16L10.5,18.5L11.92,19.92L19.84,12L11.92,4.08L10.5,5.5L16,11H4Z" /></svg>
        </div>
        <div class="toolbar-btn" onclick="window.location.reload()" title="Refresh">
            <svg viewBox="0 0 24 24"><path d="M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z" /></svg>
        </div>
    `;
    document.body.appendChild(toolbar);
})();
"#;

const INIT_JS: &str = r#"
(function() {
    // Progress reporting
    window.addEventListener('load', () => window.ipc.postMessage('progress:100'));
    window.addEventListener('beforeunload', () => window.ipc.postMessage('progress:30'));
    
    const originalSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.send = function(...args) {
        window.ipc.postMessage('progress:10');
        this.addEventListener('load', () => window.ipc.postMessage('progress:100'));
        this.setRequestHeader('Sec-CH-UA', '"Android WebView\";v=\"120\", \"Chromium\";v=\"120\", \"Not A(Brand\";v=\"24\"');
        this.setRequestHeader('Sec-CH-UA-Mobile', '?1');
        this.setRequestHeader('Sec-CH-UA-Platform', '"Android"');
        this.setRequestHeader('X-Requested-With', 'com.matchapro.app');
        return originalSend.apply(this, args);
    };
    Object.defineProperty(navigator, 'webdriver', { get: () => undefined });
    if (!window.chrome) { window.chrome = { runtime: {} }; }
    const originalScreen = window.screen;
    Object.defineProperty(window, 'screen', {
        get: () => ({ ...originalScreen, width: 390, height: 844, availWidth: 390, availHeight: 844, colorDepth: 24, pixelDepth: 24 })
    });
    if (typeof window.ontouchstart === 'undefined') { window.ontouchstart = null; }
    if (typeof navigator.maxTouchPoints === 'undefined' || navigator.maxTouchPoints === 0) {
        Object.defineProperty(navigator, 'maxTouchPoints', { get: () => 5 });
    }
})();
"#;


const FOOTER_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
<style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body { 
        height: 40px; width: 100%; margin: 0; padding: 0;
        font-family: -apple-system, system-ui, sans-serif; 
        background: #f8f9fa; border-top: 1px solid #ddd; font-size: 13px; color: #555;
        user-select: none; overflow: hidden;
    }
    .footer-container { 
        width: 100%; height: 40px; padding: 0 20px; position: relative;
    }
    .status { 
        position: absolute; left: 20px; top: 0; height: 100%; line-height: 40px;
    }
    .dot { 
        display: inline-block; width: 10px; height: 10px; border-radius: 50%; 
        margin-right: 10px; vertical-align: middle;
    }
    .online { background: #28a745; box-shadow: 0 0 5px #28a745; }
    .offline { background: #dc3545; box-shadow: 0 0 5px #dc3545; }
    
    .credits { 
        position: absolute; right: 20px; top: 0; height: 100%; line-height: 40px;
        font-weight: 500;
    }
    .credits a { color: #007bff; text-decoration: none; font-weight: 600; cursor: pointer; }
    .credits a:hover { text-decoration: underline; }
</style>
</head>
<body>
    <div class="footer-container">
        <div class="status">
            <div id="dot" class="dot offline"></div>
            <span id="text">Menghubungkan...</span>
        </div>
        <div class="credits">
            Diakali oleh <a id="github-link"> Jstfire </a> - 7415 - 1500
        </div>
    </div>
    <script>
        document.getElementById('github-link').addEventListener('click', (e) => { e.preventDefault(); window.ipc.postMessage('open_github'); });
        window.updateStatus = (online) => {
            const dot = document.getElementById('dot'), text = document.getElementById('text');
            if (online) { dot.className = 'dot online'; text.textContent = 'Terhubung'; }
            else { dot.className = 'dot offline'; text.textContent = 'Tidak Terhubung (Cek VPN)'; }
        };
    </script>
</body>
</html>
"#;








enum UserEvent { 
    UpdateStatus(bool), 
    UpdateProgress(u32),
    GoBack,
    GoForward,
    Reload,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SSL & Compatibility Flags
    #[cfg(target_os = "windows")]
    std::env::set_var(
        "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", 
        "--disable-features=IsolateOrigins,site-per-process --disable-blink-features=AutomationControlled --no-sandbox --disable-setuid-sandbox --disable-infobars --disable-dev-shm-usage --ignore-certificate-errors --allow-running-insecure-content --test-type --ignore-urlfetcher-cert-requests"
    );

    let event_loop = tao::event_loop::EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    
    let window = WindowBuilder::new()
        .with_title("Matchapro GC Desktop")
        .with_maximized(true)
        .with_decorations(false) // Frameless
        .with_window_icon(load_icon())
        .with_inner_size(LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(LogicalSize::new(900.0, 600.0))
        .build(&event_loop)?;
    let window = Arc::new(window);

    let mut web_context = WebContext::new(Some(std::env::var("LOCALAPPDATA").map(|p| std::path::PathBuf::from(p).join("MatchaproGC")).unwrap_or_else(|_| std::env::temp_dir().join("MatchaproGC"))));

    // 1. Header WebView (Controls)
    let se_black_b64 = BASE64_STANDARD.encode(SE_BLACK_BYTES);
    let header_html = HEADER_HTML.replace(
        "<!-- LOGO_PLACEHOLDER -->", 
        &format!("<img src=\"data:image/png;base64,{}\" style=\"height: 36px; width: auto; vertical-align: middle;\" />", se_black_b64)
    );

    let header_wv = WebViewBuilder::new(&window)
        .with_web_context(&mut web_context)
        .with_html(&header_html)
        .with_ipc_handler({
            let win = window.clone();
            let proxy_c = proxy.clone();
            move |msg| match msg.body().as_str() {
                "drag" => { let _ = win.drag_window(); }
                "minimize" => { win.set_minimized(true); }
                "maximize" => { win.set_maximized(!win.is_maximized()); }
                "close" => { std::process::exit(0); }
                "back" => { let _ = proxy_c.send_event(UserEvent::GoBack); }
                "forward" => { let _ = proxy_c.send_event(UserEvent::GoForward); }
                "refresh" => { let _ = proxy_c.send_event(UserEvent::Reload); }
                _ => {}
            }
        })
        .build()?;
    let header_wv = Arc::new(header_wv);

    // 2. Content WebView
    let mut main_init = format!("{}\n\n{}\n\n", INIT_JS, TOOLBAR_JS);
    main_init.push_str(&format!("window.addEventListener('DOMContentLoaded', () => {{ const s = document.createElement('style'); s.textContent = `{}`; document.head.appendChild(s); }});", CSS_STYLES.replace('`', r"\`")));

    let content_wv = WebViewBuilder::new(&window)
        .with_web_context(&mut web_context)
        .with_url(TARGET_URL)
        .with_user_agent(USER_AGENT)
        .with_initialization_script(&main_init)
        .with_ipc_handler({
            let proxy_c = proxy.clone();
            move |msg| {
                if msg.body().starts_with("progress:") {
                    if let Ok(p) = msg.body().trim_start_matches("progress:").parse::<u32>() {
                        let _ = proxy_c.send_event(UserEvent::UpdateProgress(p));
                    }
                }
            }
        })
        .with_headers(http::HeaderMap::from_iter(vec![
            (http::header::HeaderName::from_static("sec-ch-ua"), http::header::HeaderValue::from_static("\"Android WebView\";v=\"120\", \"Chromium\";v=\"120\", \"Not A(Brand\";v=\"24\"")),
            (http::header::HeaderName::from_static("sec-ch-ua-mobile"), http::header::HeaderValue::from_static("?1")),
            (http::header::HeaderName::from_static("sec-ch-ua-platform"), http::header::HeaderValue::from_static("\"Android\"")),
            (http::header::HeaderName::from_static("x-requested-with"), http::header::HeaderValue::from_static("com.matchapro.app")),
        ]))
        .with_devtools(true)
        .build()?;
    let content_wv = Arc::new(content_wv);

    // 3. Footer WebView
    let footer_wv = WebViewBuilder::new(&window)
        .with_web_context(&mut web_context)
        .with_html(FOOTER_HTML)
        .with_ipc_handler(|msg| {
            if msg.body() == "open_github" { let _ = open::that("https://github.com/Jstfire"); }
        })
        .build()?;
    let footer_wv = Arc::new(footer_wv);

    resize_views(&header_wv, &content_wv, &footer_wv, window.inner_size().width, window.inner_size().height, window.scale_factor());

    tokio::spawn(async move {
        loop {
            let online = connection::check_connection().await;
            let _ = proxy.send_event(UserEvent::UpdateStatus(online));
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = tao::event_loop::ControlFlow::Exit,
            Event::WindowEvent { event: WindowEvent::Resized(s), .. } => resize_views(&header_wv, &content_wv, &footer_wv, s.width, s.height, window.scale_factor()),
            Event::UserEvent(ev) => match ev {
                UserEvent::UpdateStatus(on) => { let _ = footer_wv.evaluate_script(&format!("if(window.updateStatus)updateStatus({});", on)); }
                UserEvent::UpdateProgress(p) => { let _ = header_wv.evaluate_script(&format!("if(window.updateProgress)updateProgress({});", p)); }
                UserEvent::GoBack => { let _ = content_wv.evaluate_script("window.history.back()"); }
                UserEvent::GoForward => { let _ = content_wv.evaluate_script("window.history.forward()"); }
                UserEvent::Reload => { let _ = content_wv.evaluate_script("window.location.reload()"); }
            }
            _ => {}
        }
    });
}

fn load_icon() -> Option<Icon> {

    let icon_bytes = include_bytes!("../logo-app.png");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Icon::from_rgba(rgba.into_raw(), width, height).ok()
    } else { None }
}

fn resize_views(h: &WebView, c: &WebView, f: &WebView, w: u32, h_total: u32, scale_factor: f64) {
    let header_height = (HEADER_HEIGHT as f64 * scale_factor) as u32;
    let footer_height = (FOOTER_HEIGHT as f64 * scale_factor) as u32;

    let _ = h.set_bounds(wry::Rect {
        position: wry::dpi::Position::Physical(wry::dpi::PhysicalPosition::new(0, 0)),
        size: wry::dpi::Size::Physical(wry::dpi::PhysicalSize::new(w, header_height)),
    });
    let main_h = if h_total > header_height + footer_height { h_total - header_height - footer_height } else { 1 };
    let _ = c.set_bounds(wry::Rect {
        position: wry::dpi::Position::Physical(wry::dpi::PhysicalPosition::new(0, header_height as i32)),
        size: wry::dpi::Size::Physical(wry::dpi::PhysicalSize::new(w, main_h)),
    });
    let _ = f.set_bounds(wry::Rect {
        position: wry::dpi::Position::Physical(wry::dpi::PhysicalPosition::new(0, (header_height + main_h) as i32)),
        size: wry::dpi::Size::Physical(wry::dpi::PhysicalSize::new(w, footer_height)),
    });
}


