use std::collections::HashMap;
use std::env;
use std::process::Command;

/// هيكل مسؤول عن تنظيف وإعداد بيئة التشغيل
pub struct EnvScrubber {
    /// المتغيرات التي يجب حذفها لأنها تسبب تعارضاً
    forbidden_vars: Vec<String>,
    /// المتغيرات التي يجب فرضها (مثل متغيرات العرض الرسومي)
    required_vars: HashMap<String, String>,
}

impl EnvScrubber {
    pub fn new() -> Self {
        Self {
            forbidden_vars: vec![
                "LD_PRELOAD".to_string(), // أخطر متغير يسبب انهيار Wine في PRoot
                "XDG_RUNTIME_DIR".to_string(),
                "DBUS_SESSION_BUS_ADDRESS".to_string(),
            ],
            required_vars: HashMap::from([
                ("WINEPREFIX".to_string(), "/home/user/.wine_default".to_string()),
                ("WINEDEBUG".to_string(), "-all".to_string()), // تقليل السجلات لزيادة الأداء
                ("DISPLAY".to_string(), ":1".to_string()),     // التوجيه لـ Termux-X11
            ]),
        }
    }

    /// دالة تنظيف البيئة الحالية وتجهيز "أمر" (Command) جديد
    pub fn prepare_command(&self, program_path: &str) -> Command {
        let mut cmd = Command::new("wine");
        
        // 1. مسح المتغيرات الضارة من البيئة الحالية
        for var in &self.forbidden_vars {
            cmd.env_remove(var);
        }

        // 2. حقن المتغيرات المطلوبة للتشغيل المستقر
        for (key, value) in &self.required_vars {
            cmd.env(key, value);
        }

        // 3. إضافة مسار البرنامج المراد تشغيله
        cmd.arg(program_path);

        cmd
    }
}
