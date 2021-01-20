use druid::{
    widget::{Button, Flex, Label, Scope, ScopeTransfer, TextBox},
    AppLauncher, Data, Env, Lens, PlatformError, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
    email: String,
}

#[derive(Clone, Data, Lens)]
struct EditState {
    name: String,
    email: String,
    saved: bool,
}

impl EditState {
    pub fn new(data: AppState) -> Self {
        Self {
            name: data.name,
            email: data.email,
            saved: false,
        }
    }
}
struct EditTransfer;

impl ScopeTransfer for EditTransfer {
    type In = AppState;

    type State = EditState;

    fn read_input(&self, state: &mut Self::State, inner: &Self::In) {
        if state.saved {
            state.name = inner.name.clone();
            state.email = inner.email.clone();
            state.saved = false;
        }
    }

    fn write_back_input(&self, state: &Self::State, inner: &mut Self::In) {
        if state.saved {
            inner.name = state.name.clone();
            inner.email = state.email.clone();
        }
    }
}

fn ui_builder() -> impl Widget<AppState> {
    let name_textbox = TextBox::new().lens(EditState::name);
    let button = Button::new("Save").on_click(|_event, data: &mut EditState, _env| {
        data.saved = true;
    });

    let layout = Flex::column().with_child(name_textbox).with_child(button);
    let scope = Scope::from_function(EditState::new, EditTransfer, layout);

    let name_label = Label::dynamic(|data: &AppState, _env: &Env| data.name.to_owned());

    Flex::column().with_child(name_label).with_child(scope)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Saving Data on Button Click");
    AppLauncher::with_window(main_window).launch(AppState {
        name: "Your Name".to_string(),
        email: "yourname@yourname.com".to_string(),
    })?;
    Ok(())
}
