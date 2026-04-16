pub enum DisplayProtocol {
    X11,
    Wayland,
}

pub fn detect_protocol() -> DisplayProtocol {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        DisplayProtocol::Wayland
    } else {
        DisplayProtocol::X11
    }
}
