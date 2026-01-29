# Release v1.5.0

## 🏷️ Tag Name
```
v1.5.0
```

## 📌 Release Title
```
Matchapro GC Desktop v1.5.0 - Rust Native Release
```

## 📝 Release Notes (Copy untuk GitHub Release)

```markdown
## 🎉 Matchapro GC Desktop v1.5.0

### Highlights
🚀 **Multi-Platform Support** - Sekarang mendukung **Windows** dan **macOS** secara native!

### ✨ Fitur Utama

- 🖥️ **Native Desktop App** - Performa lebih cepat dan ringan
- 🔄 **Navigasi Browser** - Tombol Back, Forward, Refresh
- 📊 **Progress Bar** - Indikator loading halaman
- 🌐 **Status Koneksi** - Cek koneksi realtime
- 📱 **Mobile User-Agent** - Kompatibel dengan Matchapro
- 🍎 **macOS Support** - Tersedia file .dmg dan .app portable
- ⚡ **Portable** - Tidak perlu instalasi

### 📦 Spesifikasi Aplikasi

| Aspek | Keterangan |
|-------|------------|
| Ukuran File | ~3.5 MB |
| RAM Usage | ~80 MB |
| Startup Time | ~1 detik |
| Runtime | WebView2 (built-in Windows) |

### 🔧 Persyaratan Sistem
#### Windows
- Windows 10/11 (64-bit)
- Microsoft Edge WebView2 Runtime

#### macOS
- macOS Catalina (10.15) atau lebih baru
- Support Apple Silicon (M1/M2) & Intel

- Koneksi Internet + VPN BPS (Semua Platform)

### 📥 Download

| Platform | File | Keterangan |
|----------|------|------------|
| Windows | `Matchapro-GC-Desktop-Portable-V1.5.0.exe` | Portable Executable |
| macOS | `Matchapro-GC-Desktop-Portable-V1.5.0.dmg` | macOS Portable (Drag & Drop) |
| macOS | `Matchapro-GC-Desktop-Portable-V1.5.0.app.tar.gz` | macOS Portable (Extract & Run) |

### 🚀 Cara Penggunaan
#### Windows
1. Download file `.exe`
2. Jalankan langsung

#### macOS
1. **PENTING:** Karena aplikasi ini tidak ditandatangani Apple (Unsigned), Anda mungkin tidak bisa membukanya dengan double-click biasa.
2. **Cara Buka:** 
   - **Klik Kanan** (Control+Click) pada aplikasi > Pilih **Open** > Klik **Open** pada dialog peringatan.
   - Atau gunakan perintah terminal jika aplikasi dianggap rusak: `xattr -cr /AppPath/NamaApp.app`

#### Umum
- Aktifkan VPN BPS
- Login dengan akun SSO BPS

### ⚠️ Known Issues

- Beberapa fitur mungkin tidak berfungsi tanpa VPN BPS

---

**Diakali oleh Jstfire - 7415 - 1500**
*BPS Kabupaten Buton Selatan*
```

---

## 🗂️ Assets untuk Upload

Upload file berikut ke GitHub Release:

1. `app-release/Matchapro-GC-Desktop-Portable-V1.5.0.exe`

---

## 📋 Langkah Membuat Release di GitHub

1. Buka repository di GitHub
2. Klik **"Releases"** di sidebar kanan
3. Klik **"Draft a new release"**
4. Isi:
   - **Tag version**: `v1.5.0`
   - **Release title**: `Matchapro GC Desktop v1.5.0 - Rust Native Release`
   - **Description**: Copy markdown di atas
5. Upload file `Matchapro-GC-Desktop-Portable-V1.5.0.exe` ke bagian **"Attach binaries"**
6. Centang **"Set as the latest release"**
7. Klik **"Publish release"**

---

## ✅ Checklist Sebelum Release

- [ ] Build berhasil tanpa error
- [ ] Test file .exe berjalan dengan baik
- [ ] VPN connection test passed
- [ ] README.md sudah updated
- [ ] Version di Cargo.toml sudah benar (1.5.0)
