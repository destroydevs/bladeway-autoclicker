fn main() {
    let _ = winres::WindowsResource::new()
        .set_icon("icon.ico")
        .set("FileDescription", "Автокликер BladeWay")
        .set("ProductName", "BladeWay Autoclicker")
        .set("LegalCopyright", "Copyright © 2025 BladeWay")
        .set("LegalTrademark", "BladeWay")
        .compile();
}
