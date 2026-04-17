mod core;
mod engine;

use eframe::egui;
use crate::core::{SystemContext, setup_dependencies};
use crate::engine::AppInstance;
use std::process::Command;
use std::fs;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // --- إعداد بيئة العرض (X11) بشكل آمن ---
    unsafe {
        std::env::remove_var("WAYLAND_DISPLAY");
        std::env::set_var("DISPLAY", ":1");
        std::env::set_var("QT_X11_NO_MITSHM", "1");
        std::env::set_var("_JAVA_AWT_WM_NONREPARENTING", "1");
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    // تهيئة النظام الأساسي
    setup_dependencies();

    // تشغيل مدير النوافذ لضمان عمل واجهات Wine
    let _ = Command::new("openbox")
        .arg("--replace")
        .env("DISPLAY", ":1")
        .spawn();

    let ctx = SystemContext::new();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 750.0])
            .with_title("Windows-RL Pro: Modern Edition"),
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
    // المسار الحالي للمستعرض الذكي
    current_path: PathBuf,
}

impl WindowsRLApp {
    fn new(ctx: SystemContext) -> Self {
        Self {
            ctx,
            active_apps: Vec::new(),
            logs: "System Ready (X11 Mode)".into(),
            // تحديد مسار البداية الافتراضي لـ Wine
            current_path: PathBuf::from("/root/.windows-rl/default/drive_c/"),
        }
    }
}

impl eframe::App for WindowsRLApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // تنظيف العمليات المنتهية تلقائياً
        self.active_apps.retain_mut(|(_, app)| app.is_running());

        // --- القائمة الجانبية (Navigation SidePanel) ---
        egui::SidePanel::left("main_nav").show(ctx, |ui| {
            ui.add_space(15.0);
            ui.vertical_centered(|ui| {
                ui.heading("🛡️ Windows-RL");
                ui.label(egui::RichText::new("X11 Backend Active").color(egui::Color32::GREEN).small());
            });
            
            ui.add_space(20.0);
            ui.separator();

            ui.vertical_centered_justified(|ui| {
                if ui.button("📁 Explorer Home").clicked() {
                    self.current_path = PathBuf::from("/root/.windows-rl/default/drive_c/");
                    self.logs = "Back to Drive C".into();
                }

                if ui.button("⚙️ Wine Config").clicked() {
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

        // --- اللوحة المركزية (Modern File Explorer) ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📂 Modern Explorer");
                ui.add_space(20.0);
                ui.label(egui::RichText::new(&self.logs).color(egui::Color32::GOLD));
            });

            // شريط مسار المجلد الحالي (Breadcrumbs)
            ui.horizontal(|ui| {
                if ui.button("⬅ Back").clicked() {
                    if let Some(parent) = self.current_path.parent() {
                        self.current_path = parent.to_path_buf();
                    }
                }
                ui.separator();
                ui.label(egui::RichText::new(format!("📍 {}", self.current_path.display()))
                    .monospace()
                    .color(egui::Color32::LIGHT_BLUE));
            });
            
            ui.separator();

            // عرض محتويات المجلد بشكل ديناميكي
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Ok(entries) = fs::read_dir(&self.current_path) {
                    for entry in entries.flatten() {
                        let fpath = entry.path();
                        let fname = fpath.file_name().unwrap_or_default().to_string_lossy().to_string();
                        
                        ui.horizontal(|ui| {
                            if fpath.is_dir() {
                                ui.label("📁");
                                if ui.link(&fname).clicked() {
                                    self.current_path = fpath;
                                }
                            } else {
                                let is_exe = fpath.extension().and_then(|s| s.to_str()) == Some("exe");
                                ui.label(if is_exe { "📀" } else { "📄" });
                                
                                if is_exe {
                                    if ui.button(format!("Run {}", fname)).clicked() {
                                        let app = AppInstance::launch(&self.ctx, &fpath.to_string_lossy(), "/root/.windows-rl/default");
                                        self.active_apps.push((fname.clone(), app));
                                        self.logs = format!("Launched {}", fname);
                                    }
                                } else {
                                    ui.label(&fname);
                                }
                            }
                        });
                    }
                } else {
                    ui.colored_label(egui::Color32::RED, "🚫 Error: Path not accessible.");
                }
            });
        });

        // تحديث الواجهة لضمان استجابة العمليات
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}
