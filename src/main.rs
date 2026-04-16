mod core;
mod engine;

use eframe::egui;
use crate::core::{SystemContext, setup_dependencies};
use crate::engine::AppInstance;
use std::process::Command;
use std::fs;

fn main() -> eframe::Result<()> {
    // --- الحماية القسرية: منع انهيار Winit بسبب Wayland ---
    unsafe {
        // حذف متغير وايلاند تماماً لإجبار المكتبة على استخدام X11 المستقر
        std::env::remove_var("WAYLAND_DISPLAY");
        
        // إعداد شاشة العرض رقم 1 (المتوافقة مع termux-x11 :1)
        std::env::set_var("DISPLAY", ":1");
        
        // حلول تقنية لمنع انهيار الرسوميات في بيئات الحاويات (PRoot)
        std::env::set_var("QT_X11_NO_MITSHM", "1");
        std::env::set_var("_JAVA_AWT_WM_NONREPARENTING", "1");
    }

    // تهيئة النظام (تثبيت الحزم مثل openbox و procps)
    setup_dependencies();

    // تشغيل Openbox مع خاصية --replace لإنهاء أي نسخة قديمة معلقة
    let _ = Command::new("openbox")
        .arg("--replace")
        .env("DISPLAY", ":1")
        .spawn();

    let ctx = SystemContext::new();
    
    // إعدادات واجهة المستخدم
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_title("Windows-RL Stable X11 Edition"),
        ..Default::default()
    };

    eframe::run_native(
        "Windows-RL Pro",
        options,
        Box::new(|_cc| Box::new(WindowsRLApp::new(ctx))),
    )
}

struct WindowsRLApp {
    ctx: SystemContext,
    active_apps: Vec<(String, AppInstance)>,
    logs: String,
}

impl WindowsRLApp {
    fn new(ctx: SystemContext) -> Self {
        Self {
            ctx,
            active_apps: Vec::new(),
            logs: "System Ready (X11 Mode)".into(),
        }
    }
}

impl eframe::App for WindowsRLApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // تنظيف العمليات المنتهية
        self.active_apps.retain_mut(|(_, app)| app.is_running());

        egui::SidePanel::left("main_nav").show(ctx, |ui| {
            ui.add_space(15.0);
            ui.vertical_centered(|ui| {
                ui.heading("🛡️ Windows-RL");
                ui.label(egui::RichText::new("X11 Backend Active").color(egui::Color32::GREEN).small());
            });
            
            ui.add_space(20.0);
            ui.separator();

            ui.vertical_centered_justified(|ui| {
                if ui.button("📁 Explorer").clicked() {
                    let app = AppInstance::launch(&self.ctx, "explorer.exe", "/root/.windows-rl/default");
                    self.active_apps.push(("Explorer".into(), app));
                }

                if ui.button("⚙️ Control Panel").clicked() {
                    let app = AppInstance::launch(&self.ctx, "winecfg", "/root/.windows-rl/default");
                    self.active_apps.push(("WineCfg".into(), app));
                }

                ui.add_space(10.0);
                ui.separator();

                if ui.button("🎮 Install DX9").clicked() {
                    self.logs = "Installing DirectX...".into();
                    let _ = Command::new("winetricks")
                        .arg("-q").arg("d3dx9")
                        .env("WINEPREFIX", "/root/.windows-rl/default")
                        .spawn();
                }

                ui.add_space(30.0);
                if ui.button(egui::RichText::new("🛑 STOP ALL").color(egui::Color32::RED).strong()).clicked() {
                    let _ = Command::new("pkill").arg("-9").arg("wine").spawn();
                    let _ = Command::new("pkill").arg("-9").arg("box64").spawn();
                    self.active_apps.clear();
                    self.logs = "All processes killed".into();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📂 Software on Drive C:");
            ui.label(format!("Log: {}", self.logs));
            ui.separator();

            let path = "/root/.windows-rl/default/drive_c/";
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let fpath = entry.path();
                        if fpath.extension().and_then(|s| s.to_str()) == Some("exe") {
                            let fname = fpath.file_name().unwrap().to_string_lossy();
                            ui.horizontal(|ui| {
                                ui.label("📀");
                                if ui.button(format!("Run {}", fname)).clicked() {
                                    let app = AppInstance::launch(&self.ctx, &fpath.to_string_lossy(), "/root/.windows-rl/default");
                                    let name_str = fname.to_string(); // حل مشكلة Ownership
                                    self.active_apps.push((name_str, app));
                                    self.logs = format!("Launched {}", fname);
                                }
                            });
                        }
                    }
                } else {
                    ui.colored_label(egui::Color32::GOLD, "Drive C not found. Launch Explorer first.");
                }
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
