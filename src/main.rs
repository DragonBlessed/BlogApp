use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();
    ui.on_posted(move | string | {
        let ui: AppWindow = ui_handle.unwrap();
        let posted_text = string;
        ui.set_postText(posted_text.into());
    });

    ui.run()
}
