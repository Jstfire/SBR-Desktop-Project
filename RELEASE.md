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
🚀 **Rust Native Desktop App** - Aplikasi dibangun dengan Rust + WebView2 untuk performa optimal!

### ✨ Fitur Utama

- 🖥️ **Native Desktop App** - Performa lebih cepat dan ringan
- 🔄 **Navigasi Browser** - Tombol Back, Forward, Refresh
- 📊 **Progress Bar** - Indikator loading halaman
- 🌐 **Status Koneksi** - Cek koneksi realtime
- 📱 **Mobile User-Agent** - Kompatibel dengan Matchapro
- ⚡ **Portable** - Tidak perlu instalasi

### 📦 Spesifikasi Aplikasi

| Aspek | Keterangan |
|-------|------------|
| Ukuran File | ~3.5 MB |
| RAM Usage | ~80 MB |
| Startup Time | ~1 detik |
| Runtime | WebView2 (built-in Windows) |

### 🔧 Persyaratan Sistem

- Windows 10/11 (64-bit)
- Microsoft Edge WebView2 Runtime (biasanya sudah terinstal)
- Koneksi Internet + VPN BPS

### 📥 Download

| File | Keterangan |
|------|------------|
| `Matchapro-GC-Desktop-Portable-V1.5.0.exe` | Portable executable |

### 🚀 Cara Penggunaan

1. Download file `.exe` di atas
2. Jalankan langsung (tidak perlu install)
3. Aktifkan VPN BPS
4. Login dengan akun SSO BPS

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
