const { app, BrowserWindow, session, shell, ipcMain, dialog } = require('electron');
const path = require('path');
const fs = require('fs');

// --- EXTREME COMPATIBILITY MODE (v2.9) ---
// These flags force Chromium to run in the safest, most compatible mode possible.
// Essential for "clean" PCs missing drivers or with strict security policies.

// 1. Disable Sandbox (Critical for some envs)
app.commandLine.appendSwitch('no-sandbox');
// 2. Disable GPU (Prevents White Screen)
app.commandLine.appendSwitch('disable-gpu');
app.disableHardwareAcceleration();
// 3. Disable Software Rasterizer (Prevents WebGL crash)
app.commandLine.appendSwitch('disable-software-rasterizer');
// 4. Memory Management
app.commandLine.appendSwitch('disable-dev-shm-usage');

// --- CONSTANTS ---
const APP_TITLE = 'Matchapro GC Desktop';
const TARGET_URL = 'https://matchapro.web.bps.go.id/login';

// --- DYNAMIC PARTITIONING (v2.8) ---
const DYNAMIC_PARTITION = `ram:matchapro-session-${Date.now()}-${Math.floor(Math.random() * 10000)}`;

const USER_AGENT = 'Mozilla/5.0 (Linux; Android 12; M2010J19CG Build/SKQ1.211202.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/120.0.0.0 Mobile Safari/537.36';

const EXTRA_HEADERS = {
    'Sec-CH-UA': '"Android WebView";v="120", "Chromium";v="120", "Not A(Brand";v="24"',
    'Sec-CH-UA-Mobile': '?1',
    'Sec-CH-UA-Platform': '"Android"',
    'X-Requested-With': 'com.matchapro.app'
};

let mainWindow;

// Certificate handling
app.commandLine.appendSwitch('ignore-certificate-errors');
app.commandLine.appendSwitch('allow-running-insecure-content');

function createWindow() {
    const ses = session.fromPartition(DYNAMIC_PARTITION);

    ses.webRequest.onBeforeSendHeaders((details, callback) => {
        details.requestHeaders['User-Agent'] = USER_AGENT;
        Object.keys(EXTRA_HEADERS).forEach(key => {
            details.requestHeaders[key] = EXTRA_HEADERS[key];
        });
        callback({ requestHeaders: details.requestHeaders });
    });

    mainWindow = new BrowserWindow({
        width: 1280,
        height: 800,
        minWidth: 900,
        minHeight: 600,
        title: APP_TITLE,
        icon: path.join(__dirname, 'logo-app.png'),
        backgroundColor: '#ffffff',
        webPreferences: {
            nodeIntegration: false,
            contextIsolation: true,
            preload: path.join(__dirname, 'preload.js'),
            partition: DYNAMIC_PARTITION,
            backgroundThrottling: false
        },
        autoHideMenuBar: true,
        show: false
    });

    mainWindow.webContents.setUserAgent(USER_AGENT);

    const stylesPath = path.join(__dirname, 'src', 'styles.css');
    let cssContent = '';
    try {
        cssContent = fs.readFileSync(stylesPath, 'utf8');
    } catch (e) { }

    mainWindow.webContents.on('did-finish-load', () => {
        if (cssContent) mainWindow.webContents.insertCSS(cssContent);
    });

    // Flag to allow native error pages (breaking the splash loop)
    let suppressSplashOnError = false;

    // --- CRASH HANDLER (v2.9) ---
    // ... (unchanged)

    mainWindow.webContents.on('did-fail-load', (event, errorCode, errorDescription) => {
        // Log for debugging
        console.log(`[Main] Page failed to load. Code: ${errorCode}, Desc: ${errorDescription}`);

        // If suppressed (Force Reload), do NOT show splash. Let browser show error.
        if (suppressSplashOnError) {
            console.log('[Main] Splash suppressed. Showing native error page.');
            suppressSplashOnError = false; // Reset for next time
            return;
        }

        // Show splash screen for ANY error except used cancellation (-3)
        // This covers -105 (Not Resolved), -106 (Disconnected), -21 (Network Changed), etc.
        if (errorCode !== -3) {
            console.log('[Main] Loading splash screen file...');
            // Robust fix: Load local file instead of relying on IPC overlay
            mainWindow.loadFile('splash.html');
        }
    });

    // ... (unchanged)

    // ...

    // Force reload handler removed
    // ipcMain.on('force-reload', () => { ... });

    // Handle Connection Check (Pre-flight) - Using correct session
    ipcMain.handle('check-connection', async () => {
        // ... (existing implementation)

        mainWindow.loadURL(TARGET_URL);

        mainWindow.once('ready-to-show', () => {
            mainWindow.show();
            mainWindow.maximize();
        });

        mainWindow.webContents.setWindowOpenHandler(({ url }) => {
            if (url.includes('bps.go.id') || url.includes('google.com') || url.includes('accounts')) {
                return { action: 'allow' };
            }
        });
    });

    // Initial load
    mainWindow.loadURL(TARGET_URL);

    mainWindow.once('ready-to-show', () => {
        mainWindow.show();
    });
}

ipcMain.on('OPEN_EXTERNAL', (event, url) => {
    shell.openExternal(url);
});

// Reload handlers removed per user request

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit();
});

app.on('activate', () => {
    if (!mainWindow) createWindow();
});
