use druid::{
    widget::{Flex, Label, TextBox},
    AppLauncher, Data, Env, Lens, PlatformError, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
}
fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Lens Macro");
    AppLauncher::with_window(main_window).launch(AppState {
        name: "Your Name".to_string(),
    })?;
    Ok(())
}

fn ui_builder() -> impl Widget<AppState> {
    let name_label = Label::dynamic(|data: &String, _: &Env| data.clone()).lens(AppState::name);
    let name_textbox = TextBox::new().lens(AppState::name);
    Flex::column()
        .with_child(name_label)
        .with_child(name_textbox)
}
