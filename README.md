# Matchapro GC Desktop

![Version](https://img.shields.io/badge/version-1.5.1-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-brightgreen)
![Language](https://img.shields.io/badge/built%20with-Rust-orange)

> Aplikasi desktop native untuk mengakses **matchapro.web.bps.go.id** - Sensus Ekonomi 2026

---

## 📦 Release

### v1.5.0 - Latest Release

| File | Keterangan |
|------|------------|
| `Matchapro-GC-Desktop-Portable-V1.5.0.exe` | Windows Portable (exe) |
| `Matchapro-GC-Desktop-Portable-V1.5.0.dmg` | macOS Portable Image (Drag & Drop) |
| `Matchapro-GC-Desktop-Portable-V1.5.0.app.tar.gz` | macOS Portable Bundle (Extract & Run) |

**Download:** Lihat folder `app-release/` atau halaman [Releases](https://github.com/Jstfire/SBR-Desktop-Project/releases)

---

## ✨ Fitur

- 🖥️ **Native Desktop App** - Dibangun dengan Rust + WebView2 untuk performa optimal
- 🔄 **Navigasi Browser** - Tombol Back, Forward, dan Refresh
- 📊 **Progress Bar** - Indikator loading halaman
- 🌐 **Status Koneksi** - Cek koneksi realtime di footer
- 📱 **Mobile User-Agent** - Emulasi Android WebView untuk kompatibilitas Matchapro
- 🎨 **Custom Titlebar** - Header dengan kontrol window (minimize, maximize, close)
- ⚡ **Portable** - Tidak perlu instalasi, langsung jalankan .exe
- 💾 **Persistent Session** - Data login tersimpan di LocalAppData

---

## 🖼️ Screenshot

Aplikasi menampilkan:
- **Header**: Logo SE2026 + judul aplikasi + kontrol navigasi + window controls
- **Content**: WebView menampilkan halaman Matchapro
- **Footer**: Status koneksi + kredit developer

---

## 🔧 Persyaratan Sistem

- **OS**: Windows 10/11 (64-bit)
- **Runtime**: Microsoft Edge WebView2 Runtime (biasanya sudah terinstal)
- **Koneksi**: Internet + VPN BPS (jika akses dari luar jaringan BPS)

---

## 🚀 Cara Penggunaan

### Windows
1. Download file `.exe` dari folder `app-release/`
2. Jalankan file tersebut (double-click)
3. Pastikan VPN BPS aktif jika akses dari luar jaringan kantor
4. Login menggunakan akun SSO BPS

### macOS
1. Download file `.dmg` (rekomendasi) atau `.app.tar.gz`
2. **Cara Menjalankan:**
   - **Metode 1 (Paling Mudah):** Klik kanan (atau Control-Click) pada aplikasi, lalu pilih **Open**. Jika muncul peringatan, klik **Open** lagi.
   - **Metode 2 (Jika Error "App is damaged"):** Buka Terminal, ketik perintah berikut, lalu drag aplikasi ke terminal:
     ```bash
     xattr -cr /path/to/Matchapro\ GC\ Desktop.app
     ```
3. Pastikan VPN BPS aktif.

---

## 🛠️ Build dari Source

### Prasyarat
- [Rust](https://rustup.rs/) (latest stable)
- Windows SDK

### Langkah Build

```bash
# Clone repository
git clone https://github.com/Jstfire/SBR-Desktop-Project.git
cd SBR-Desktop-Project

### Langkah Build

#### Windows
```bash
cargo build --release
```

#### macOS
```bash
# Build binary standard
cargo build --release

# Build app bundle / dmg (requires cargo-bundle)
cargo install cargo-bundle
cargo bundle --release
```

# File output ada di target/release/bundle/
```

---

## 📁 Struktur Proyek

```
SBR-Desktop-Project/
├── src/
│   ├── main.rs          # Entry point & WebView setup
│   ├── connection.rs    # Connection checker
│   └── styles.css       # Injected CSS styles
├── app-release/         # Compiled executables
├── build.rs             # Build script (icon embedding)
├── Cargo.toml           # Rust dependencies
├── logo-app.ico         # Application icon
├── logo-app.png         # App icon (PNG)
├── logo-se.png          # SE2026 logo
└── se-black.png         # Header logo
```

---

## 📋 Tech Stack

| Komponen | Library |
|----------|---------|
| Window Management | [tao](https://crates.io/crates/tao) v0.30 |
| WebView | [wry](https://crates.io/crates/wry) v0.43 |
| Async Runtime | [tokio](https://crates.io/crates/tokio) |
| HTTP Client | [reqwest](https://crates.io/crates/reqwest) |
| Image Processing | [image](https://crates.io/crates/image) |

---

## 📝 Changelog

### v1.5.0
- Rilis pertama versi Rust native
- Custom header dengan navigasi browser
- Progress bar loading
- Status koneksi realtime
- Ukuran file ringan (~3.5MB)
- Performa cepat dan efisien

---

## ⚠️ Troubleshooting

| Masalah | Solusi |
|---------|--------|
| Tidak bisa buka aplikasi | Install [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) |
| Halaman tidak loading | Pastikan VPN BPS aktif |
| Status "Tidak Terhubung" | Cek koneksi internet dan VPN |

---

## 📄 License

MIT License - Bebas digunakan untuk keperluan kantor BPS.

---

## 👨‍💻 Kredit

**Diakali oleh Jstfire - 7415 - 1500, saat ini di BPS Kabupaten Buton Selatan**

[![GitHub](https://img.shields.io/badge/GitHub-Jstfire-181717?logo=github)](https://github.com/Jstfire)
