use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DisplayProtocol {
    X11,
    Wayland,
}

#[derive(Clone)]
pub struct SystemContext {
    pub is_android: bool,
    pub _protocol: DisplayProtocol,
    pub wine_bin: PathBuf,
}

impl SystemContext {
    pub fn new() -> Self {
        let is_android = std::path::Path::new("/system").exists() || 
                        std::env::var("TERMUX_VERSION").is_ok();
        
        let protocol = if std::env::var("WAYLAND_DISPLAY").is_ok() {
            DisplayProtocol::Wayland
        } else {
            DisplayProtocol::X11
        };

        // مسار Wine الافتراضي في PRoot
        let wine_bin = if is_android {
            PathBuf::from("/root/wine-9.0-amd64/bin/wine64")
        } else {
            PathBuf::from("wine64")
        };

        Self { is_android, _protocol: protocol, wine_bin }
    }
}

pub fn setup_dependencies() {
    println!("🔍 Checking for missing components...");
    let pkgs = vec![
        "pulseaudio-utils", // للصوت
        "openbox",          // مدير النوافذ (Title Bar)
        "winetricks",       // لتحميل ملفات ويندوز
        "libgl1-mesa-dri",  // للجرافيك المستقر
        "procps",            // لإدارة المهام
        "mesa-utils"        // أدوات الفحص
    ];

    for pkg in pkgs {
        let check = Command::new("dpkg").arg("-s").arg(pkg).output();
        if check.is_err() || !check.unwrap().status.success() {
            println!("📦 Installing {}...", pkg);
            let _ = Command::new("apt").arg("install").arg("-y").arg(pkg).status();
        }
    }
}
