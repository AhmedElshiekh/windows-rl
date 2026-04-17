use std::collections::HashMap;
use std::process::Command;

pub struct EnvScrubber {
    forbidden_vars: Vec<String>,
    required_vars: HashMap<String, String>,
}

impl EnvScrubber {
    pub fn new() -> Self {
        Self {
            forbidden_vars: vec![
                "LD_PRELOAD".into(),
                "XDG_RUNTIME_DIR".into(),
                "WAYLAND_DISPLAY".into(),
            ],
            required_vars: HashMap::from([
                ("DISPLAY".into(), ":1".into()),
                ("WINIT_UNIX_BACKEND".into(), "x11".into()),
            ]),
        }
    }

    pub fn apply(&self, cmd: &mut Command) {
        for var in &self.forbidden_vars {
            cmd.env_remove(var);
        }
        for (key, value) in &self.required_vars {
            cmd.env(key, value);
        }
    }
}
