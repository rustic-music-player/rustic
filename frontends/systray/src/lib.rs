use failure::Error;

#[cfg(target_os = "windows")]
pub fn start() -> Result<(), Error> {
    let mut app = systray::Application::new()?;
    let icon = include_bytes!("../../../assets/logo.png");
    app.set_icon_from_buffer(icon, 1000, 1000)?;
    app.set_tooltip("Rustic")?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn start() -> Result<(), Error> {
    Ok(())
}
