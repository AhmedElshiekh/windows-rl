use std::process::{Command, Child};
use crate::core::SystemContext;

pub struct AppInstance {
    pub child: Child,
}

impl AppInstance {
    pub fn launch(ctx: &SystemContext, exe: &str, prefix: &str) -> Self {
        let mut cmd = if ctx.is_android {
            let mut c = Command::new("/usr/bin/box64");
            c.arg(&ctx.wine_bin);
            
            // تحسينات الأداء لـ Box64
            c.env("BOX64_DYNAREC", "1")
             .env("BOX64_LOG", "0")
             .env("BOX64_DYNAREC_STRONGMEM", "1");
            c
        } else {
            Command::new("wine64")
        };

        // وضع الجرافيك المستقر لمنع انهيار النوافذ
        if ctx.is_android {
            cmd.env("LIBGL_ALWAYS_SOFTWARE", "1")
               .env("GALLIUM_DRIVER", "llvmpipe")
               .env("MESA_GL_VERSION_OVERRIDE", "4.5");
        }

        cmd.arg(exe)
           .env("WINEPREFIX", prefix)
           .env("DISPLAY", ":1")
           .env("PULSE_SERVER", "127.0.0.1")
           .env("WINEDEBUG", "-all");

        let child = cmd.spawn().expect("❌ Failed to launch program");
        Self { child }
    }

    pub fn is_running(&mut self) -> bool {
        self.child.try_wait().map(|s| s.is_none()).unwrap_or(false)
    }
}
