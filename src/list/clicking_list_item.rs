use std::sync::Arc;

use druid::{
    widget::{
        Controller, ControllerHost, Flex, Label, List, ListIter, Painter,
        Scroll, SizedBox,
    },
    AppLauncher, Color, Command, Data, Env, Event, Lens, PlatformError,
    RenderContext, Selector, Target, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    libraries: Arc<Vec<Library>>,
    selected: Option<usize>,
}

#[derive(Clone, Data, Lens)]
struct Library {
    name: String,
    downloads: u32,
}

impl Library {
    fn new(name: impl Into<String>, downloads: u32) -> Self {
        let name = name.into();
        Self { name, downloads }
    }
}

impl ListIter<(Library, Option<usize>, usize)> for AppState {
    fn for_each(
        &self,
        mut cb: impl FnMut(&(Library, Option<usize>, usize), usize),
    ) {
        for (i, library) in self.libraries.iter().enumerate() {
            cb(&(library.clone(), self.selected, i), i);
        }
    }

    fn for_each_mut(
        &mut self,
        mut cb: impl FnMut(&mut (Library, Option<usize>, usize), usize),
    ) {
        let mut any_changed = false;
        let mut libraries = Vec::with_capacity(self.libraries.len());
        for (i, library) in self.libraries.iter().enumerate() {
            let mut list_item = (library.clone(), self.selected, i);
            cb(&mut list_item, i);
            if !list_item.0.same(library) {
                any_changed = true;
            }
            libraries.push(list_item.0);
        }
        if any_changed {
            self.libraries = Arc::new(libraries);
        }
    }

    fn data_len(&self) -> usize {
        self.libraries.len()
    }
}

fn ui_builder() -> impl Widget<AppState> {
    let list = Scroll::new(
        List::new(|| {
            let name = Label::new(
                |(library, _selected, _idx): &(
                    Library,
                    Option<usize>,
                    usize,
                ),
                 _env: &Env| { library.name.clone() },
            );
            let downloads = Label::new(
                |(library, _selected, _idx): &(
                    Library,
                    Option<usize>,
                    usize,
                ),
                 _env: &Env| {
                    format!("Downloaded {} many times", library.downloads)
                },
            );
            let layout = Flex::column().with_child(name).with_child(downloads);
            let paint = Painter::new(|ctx, (_library, selected, idx), _env| {
                let is_hot = ctx.is_hot();
                let is_selected = if let Some(index) = selected {
                    index == idx
                } else {
                    false
                };
                let is_active = ctx.is_active();

                let background_color = if is_selected {
                    Color::rgb8(0x88, 0x88, 0x88)
                } else if is_active {
                    Color::rgb8(0x66, 0x66, 0x66)
                } else if is_hot {
                    Color::rgb8(0xbb, 0xbb, 0xbb)
                } else {
                    Color::rgb8(0x22, 0x22, 0x22)
                };
                let rect = ctx.size().to_rect();
                ctx.stroke(rect, &background_color, 0.);
                ctx.fill(rect, &background_color);
            });
            layout.padding(1.).background(paint).on_click(
                |event, (_library, _selected, idx), _env| {
                    event.submit_command(Command::new(
                        CHANGE_SELECTED,
                        *idx,
                        Target::Auto,
                    ));
                },
            )
        })
        .with_spacing(5.),
    )
    .expand_width();
    ControllerHost::new(list, AppController)
}

struct AppController;

impl Controller<AppState, SizedBox<AppState>> for AppController {
    fn event(
        &mut self,
        child: &mut SizedBox<AppState>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(selector) if selector.is(CHANGE_SELECTED) => {
                let selected = selector.get_unchecked(CHANGE_SELECTED);
                data.selected = Some(*selected);
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}

const CHANGE_SELECTED: Selector<usize> =
    Selector::new("my_program-change_selected_item");

fn main() -> Result<(), PlatformError> {
    let main_window =
        WindowDesc::new(ui_builder).title("Saving Data on Button Click");
    let libraries = Arc::new(vec![
        Library::new("rand", 51_977_492),
        Library::new("syn", 46_944_674),
        Library::new("libc", 43_103_104),
        Library::new("quote", 41_305_395),
        Library::new("rand-core", 39_592_178),
        Library::new("unicode-xid", 38_312_723),
        Library::new("proc-macro2", 37_951_701),
        Library::new("bitflags", 35_557_922),
        Library::new("serde", 35_270_662),
        Library::new("log", 35_193_490),
    ]);
    AppLauncher::with_window(main_window).launch(AppState {
        libraries,
        selected: None,
    })?;
    Ok(())
}
