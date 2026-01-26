const { contextBridge, ipcRenderer } = require('electron');

// Expose safe API to renderer process
contextBridge.exposeInMainWorld('electronAPI', {
    reloadPage: () => ipcRenderer.send('reload-page'),
    forceReload: () => ipcRenderer.send('force-reload'),
    clearCacheAndReload: () => ipcRenderer.send('clear-cache-and-reload'),
    clearCache: () => ipcRenderer.invoke('clear-cache'),
    checkConnection: () => ipcRenderer.invoke('check-connection'),
    onShowSplash: (callback) => ipcRenderer.on('show-splash', callback),
    onHideSplash: (callback) => ipcRenderer.on('hide-splash', callback)
});

// --- UI INJECTION LOGIC (v3.3.5 / v3.4.0) ---

// 1. Layout Elements: Inject when DOM is ready
function injectLayoutUI() {
    // Check if we are on the splash screen itself
    const isSplashPage = window.location.href.includes('splash.html');

    // Create splash screen (SKIP on splash page)
    let splash = null;
    if (!isSplashPage) {
        splash = document.createElement('div');
        splash.id = 'electron-splash-screen';
        splash.innerHTML = `
            <div class="splash-content">
                <div class="splash-logo-container">
                    <div class="splash-icon">⚠️</div>
                </div>
                <div class="splash-message">
                    <h2>Tidak dapat terhubung</h2>
                    <p>Periksa koneksi intenet dan VPN anda 🙏<br>Silahkan tutup aplikasi desktop ini lalu coba lagi.</p>
                </div>
            </div>
        `;
    }

    // Create Footer
    const footer = document.createElement('div');
    footer.id = 'electron-footer';
    footer.innerHTML = `
        <div class="footer-left">
            <span class="status-dot"></span>
            <span id="connection-status">Memeriksa...</span>
        </div>
        <div class="footer-right">
            Diakali oleh <a href="#" id="jstfire-link"> Jstfire </a> - 7415 - 1500
        </div>
    `;

    // Inject UI elements
    const target = document.body || document.documentElement;

    if (target) {
        if (splash) target.appendChild(splash);
        target.appendChild(footer);

        // Footer Logic
        const statusText = footer.querySelector('#connection-status');
        const statusDot = footer.querySelector('.status-dot');

        const updateStatus = () => {
            // Force "Tidak Terhubung" (Red) if on splash page
            if (isSplashPage) {
                statusText.textContent = 'Tidak Terhubung';
                statusText.style.color = '#d32f2f'; // Red
                statusDot.style.backgroundColor = '#d32f2f';
                return;
            }

            if (navigator.onLine) {
                statusText.textContent = 'Terhubung';
                statusText.style.color = '#2e7d32'; // Green
                statusDot.style.backgroundColor = '#2e7d32';
            } else {
                statusText.textContent = 'Terputus';
                statusText.style.color = '#d32f2f'; // Red
                statusDot.style.backgroundColor = '#d32f2f';
            }
        };

        window.addEventListener('online', updateStatus);
        window.addEventListener('offline', updateStatus);
        updateStatus();

        // Footer Link Handler
        footer.querySelector('#jstfire-link').addEventListener('click', (e) => {
            e.preventDefault();
            ipcRenderer.send('OPEN_EXTERNAL', 'https://github.com/Jstfire');
        });
    }

    // Splash overlay visibility events
    if (splash) {
        window.electronAPI.onShowSplash(() => {
            splash.style.display = 'flex';
        });
        window.electronAPI.onHideSplash(() => {
            splash.style.display = 'none';
        });
    }
}

// --- EXECUTION ---

if (document.readyState === 'loading') {
    window.addEventListener('DOMContentLoaded', () => {
        injectLayoutUI();
    });
} else {
    injectLayoutUI();
}
