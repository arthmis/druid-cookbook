use druid::{
    widget::{Flex, Label},
    AppLauncher, Data, Env, Lens, LensExt, PlatformError, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    miles: f64,
    fps: u32,
}

fn ui_builder() -> impl Widget<AppState> {
    let miles_label =
        Label::dynamic(|data: &String, _env: &Env| data.to_owned()).lens(AppState::miles.map(
            |miles| miles.to_string(),
            |miles, data| *miles = data.parse().expect("Should be a number"),
        ));
    let fps_label =
        Label::dynamic(|data: &String, _env: &Env| data.to_owned()).lens(AppState::fps.map(
            |fps| format!("Druid will run at {}fps one day", fps),
            |_fps, _data| {},
        ));
    Flex::column().with_child(miles_label).with_child(fps_label)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Saving Data on Button Click");
    AppLauncher::with_window(main_window).launch(AppState {
        miles: 45.3,
        fps: 360,
    })?;
    Ok(())
}
