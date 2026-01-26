fn main() {
    #[cfg(target_os = "windows")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let png_path = std::path::Path::new(&manifest_dir).join("logo-app.png");
        let ico_path = std::path::Path::new(&manifest_dir).join("logo-app.ico");

        // Force regeneration of ICO if PNG exists and (ICO doesn't exist or is empty or PNG is newer)
        let should_generate = if !ico_path.exists() || std::fs::metadata(&ico_path).map(|m| m.len()).unwrap_or(0) == 0 {
            true
        } else {
            let png_meta = std::fs::metadata(&png_path);
            let ico_meta = std::fs::metadata(&ico_path);
            if let (Ok(p), Ok(i)) = (png_meta, ico_meta) {
                p.modified().unwrap() > i.modified().unwrap()
            } else {
                true
            }
        };

        if should_generate && png_path.exists() {
            if let Ok(img) = image::open(&png_path) {
                // Ensure we use the ICO format explicitly
                let resized = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
                if let Ok(mut f) = std::fs::File::create(&ico_path) {
                    let _ = resized.write_to(&mut f, image::ImageFormat::Ico);
                    println!("cargo:warning=Regenerated logo-app.ico from logo-app.png (Size: {} bytes)", std::fs::metadata(&ico_path).map(|m| m.len()).unwrap_or(0));
                }
            }
        }

        let mut res = winres::WindowsResource::new();
        res.set("ProductName", "Matchapro GC Desktop");
        res.set("FileDescription", "Desktop application untuk mengakses matchapro.web.bps.go.id");
        res.set("CompanyName", "Jstfire - 7415 - 1500");
        res.set("LegalCopyright", "MIT License");
        res.set("FileVersion", "1.8.0");
        res.set("ProductVersion", "1.8.0");
        
        if ico_path.exists() {
            res.set_icon(ico_path.to_str().unwrap());
        } else if png_path.exists() {
            res.set_icon(png_path.to_str().unwrap());
        }

        if let Err(e) = res.compile() { 
            println!("cargo:warning=Failed to compile resources: {}", e); 
        }
        
        // Re-run if logo changes
        println!("cargo:rerun-if-changed=logo-app.png");
        println!("cargo:rerun-if-changed=build.rs");
    }
}
