use tao::{
    event::{Event, WindowEvent},
};
use wry::{WebViewBuilder, WebContext};
use std::sync::Arc;
use std::time::Duration;
use base64::prelude::*;
use crate::connection;

const SE_BLACK_BYTES: &[u8] = include_bytes!("../se-black.png");
const TARGET_URL: &str = "https://matchapro.web.bps.go.id/login";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 12; M2010J19CG Build/SKQ1.211202.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/120.0.0.0 Mobile Safari/537.36";
const CSS_STYLES: &str = include_str!("styles_mac.css");

pub async fn run(window: Arc<tao::window::Window>, event_loop: tao::event_loop::EventLoop<crate::UserEvent>, proxy: tao::event_loop::EventLoopProxy<crate::UserEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = dirs::home_dir().map(|h| h.join("Library/Application Support/MatchaproGC"));
    let mut web_context = WebContext::new(data_dir.or_else(|| Some(std::env::temp_dir().join("MatchaproGC"))));

    let se_black_b64 = BASE64_STANDARD.encode(SE_BLACK_BYTES);

    // Injection script:
    // 1. Adds CSS
    // 2. Waits for DOMContentLoaded (or runs immediately if ready)
    // 3. Appends Header/Footer to BODY (important for visibility over page content)
    // 4. Sets padding on Body to prevent content overlap
    let injection_script = format!(r#"
    (function() {{
        const injectUI = () => {{
            if (document.getElementById('app-header')) return;

            // 1. CSS
            const style = document.createElement('style');
            style.textContent = `{css}`;
            document.head.appendChild(style);

            // 2. Body Padding (Use !important to override page styles)
            const addPadding = () => {
                // Only padding body to avoid double spacing if html also has it
                document.body.style.setProperty('padding-top', '48px', 'important');
                document.body.style.setProperty('padding-bottom', '40px', 'important');
            };
            addPadding();

            // 3. Header HTML
            const header = document.createElement('div');
            header.id = 'app-header';
            header.innerHTML = `
                <div class="drag-area"></div>
                <div class="title">
                    <img src="data:image/png;base64,{logo}" />
                    <span>Matchapro GC Desktop</span>
                </div>
                <div class="browser-controls">
                    <div class="nav-btn" role="back" title="Back"><svg viewBox="0 0 24 24"><path d="M20,11V13H8L13.5,18.5L12.08,19.92L4.16,12L12.08,4.08L13.5,5.5L8,11H20Z" /></svg></div>
                    <div class="nav-btn" role="forward" title="Forward"><svg viewBox="0 0 24 24"><path d="M4,11V13H16L10.5,18.5L11.92,19.92L19.84,12L11.92,4.08L10.5,5.5L16,11H4Z" /></svg></div>
                    <div class="nav-btn" role="refresh" title="Refresh"><svg viewBox="0 0 24 24"><path d="M17.65,6.35C16.2,4.9 14.21,4 12,4A8,8 0 0,0 4,12A8,8 0 0,0 12,20C15.73,20 18.84,17.45 19.73,14H17.65C16.83,16.33 14.61,18 12,18A6,6 0 0,1 6,12A6,6 0 0,1 12,6C13.66,6 15.14,6.69 16.22,7.78L13,11H20V4L17.65,6.35Z" /></svg></div>
                </div>
                <!-- Native MacOS controls used instead -->
                <div id="app-progress-container"><div id="app-progress-bar"></div></div>
            `;
            // Append to BODY
            document.body.appendChild(header);

            // 4. Footer HTML
            const footer = document.createElement('div');
            footer.id = 'app-footer';
            footer.innerHTML = `
                <div class="status">
                    <div id="status-dot" class="dot offline"></div>
                    <span id="status-text">Menghubungkan...</span>
                </div>
                <div class="credits">Diakali oleh <a id="github-link"> Jstfire </a> - 7415 - 1500</div>
            `;
            // Append to BODY
            document.body.appendChild(footer);

            // 5. Event Listeners
            header.addEventListener('click', (e) => {{
                const btn = e.target.closest('[role]');
                if (!btn) return;
                const role = btn.getAttribute('role');
                if (role === 'back') window.history.back();
                if (role === 'forward') window.history.forward();
                if (role === 'refresh') window.location.reload();
                if (['minimize', 'maximize', 'close'].includes(role)) window.ipc.postMessage(role);
            }});
            header.querySelector('.drag-area').addEventListener('mousedown', (e) => {{ if(e.button===0) window.ipc.postMessage('drag'); }});
            const gh = document.getElementById('github-link');
            if(gh) gh.addEventListener('click', (e)=>{{ e.preventDefault(); window.ipc.postMessage('open_github'); }});

            // 6. Signal ready
            window.ipc.postMessage('ui_injected');
        }};

        // Logic to run injection
        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', injectUI);
        }} else {{
            injectUI();
        }}
        
        // Redundant observer to ensure UI stays present if wiped by page
        const observer = new MutationObserver(() => {{
            if (!document.getElementById('app-header') && document.body) {{
                injectUI();
            }}
        }});
        if(document.body) observer.observe(document.body, {{ childList: true }});
        else document.addEventListener('DOMContentLoaded', () => observer.observe(document.body, {{ childList: true }}));

        window.updateStatus = (online) => {{
            const dot = document.getElementById('status-dot');
            const text = document.getElementById('status-text');
            if(dot && text) {{
                if(online) {{ dot.className='dot online'; text.textContent='Terhubung'; }}
                else {{ dot.className='dot offline'; text.textContent='Tidak Terhubung (Cek VPN)'; }}
            }}
        }};
        window.updateProgress = (p) => {{
            const bar = document.getElementById('app-progress-bar');
            if(bar) {{
                if(p>=100) {{ bar.style.width='100%'; setTimeout(()=>{{ bar.style.opacity='0'; setTimeout(()=>{{ bar.style.width='0%'; bar.style.opacity='1'; }},500); }},500); }}
                else {{ bar.style.opacity='1'; bar.style.width=p+'%'; }}
            }}
        }};
        window.addEventListener('load', () => window.ipc.postMessage('progress:100'));
        const originalSend = XMLHttpRequest.prototype.send;
        XMLHttpRequest.prototype.send = function(...args) {{
            window.ipc.postMessage('progress:10');
            this.addEventListener('load', () => window.ipc.postMessage('progress:100'));
            this.setRequestHeader('X-Requested-With', 'com.matchapro.app');
            return originalSend.apply(this, args);
        }};
    }})();
    "#, css = CSS_STYLES.replace('`', r"\`"), logo = se_black_b64);

    let content_wv = WebViewBuilder::new(&window)
        .with_web_context(&mut web_context)
        .with_url(TARGET_URL)
        .with_user_agent(USER_AGENT)
        .with_initialization_script(&injection_script) 
        .with_ipc_handler({
            let win = window.clone();
            let proxy_c = proxy.clone();
            move |msg| {
                match msg.body().as_str() {
                    "drag" => { let _ = win.drag_window(); }
                    "minimize" => { win.set_minimized(true); }
                    "maximize" => { win.set_maximized(!win.is_maximized()); }
                    "close" => { std::process::exit(0); }
                    "open_github" => { let _ = open::that("https://github.com/Jstfire"); }
                    "ui_injected" => { 
                         // Check if we need to force maximize here?
                         // On some macOS versions, start_maximized in WindowBuilder isn't enough for frameless
                         win.set_maximized(true);
                    }
                    s if s.starts_with("progress:") => {
                        if let Ok(p) = s.trim_start_matches("progress:").parse::<u32>() {
                            let _ = proxy_c.send_event(crate::UserEvent::UpdateProgress(p));
                        }
                    }
                    _ => {}
                }
            }
        })
        .with_headers(http::HeaderMap::from_iter(vec![
            (http::header::HeaderName::from_static("sec-ch-ua"), http::header::HeaderValue::from_static("\"Android WebView\";v=\"120\", \"Chromium\";v=\"120\", \"Not A(Brand\";v=\"24\"")),
            (http::header::HeaderName::from_static("sec-ch-ua-mobile"), http::header::HeaderValue::from_static("?1")),
            (http::header::HeaderName::from_static("sec-ch-ua-platform"), http::header::HeaderValue::from_static("\"Android\"")),
            (http::header::HeaderName::from_static("x-requested-with"), http::header::HeaderValue::from_static("com.matchapro.app")),
        ]))
        .build()?;
    let content_wv = Arc::new(content_wv);

    // Initial explicit maximize (double-tap to be sure)
    window.set_maximized(true);

    let proxy_checker = proxy.clone();
    tokio::spawn(async move {
        loop {
            let online = connection::check_connection().await;
            let _ = proxy_checker.send_event(crate::UserEvent::UpdateStatus(online));
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = tao::event_loop::ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = tao::event_loop::ControlFlow::Exit,
            Event::UserEvent(ev) => match ev {
                crate::UserEvent::UpdateStatus(on) => { let _ = content_wv.evaluate_script(&format!("if(window.updateStatus)updateStatus({});", on)); }
                crate::UserEvent::UpdateProgress(p) => { let _ = content_wv.evaluate_script(&format!("if(window.updateProgress)updateProgress({});", p)); }
                crate::UserEvent::GoBack => { let _ = content_wv.evaluate_script("window.history.back()"); }
                crate::UserEvent::GoForward => { let _ = content_wv.evaluate_script("window.history.forward()"); }
                crate::UserEvent::Reload => { let _ = content_wv.evaluate_script("window.location.reload()"); }
            }
            _ => {}
        }
    });
}
