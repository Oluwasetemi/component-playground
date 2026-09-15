use component_playground_ui::{open_about_dialog, RootView};
use gpui::{
    actions, prelude::*, px, size, App, AssetSource, Bounds, KeyBinding, Menu, MenuItem, QuitMode,
    Result, SharedString, TitlebarOptions, WindowBackgroundAppearance, WindowBounds, WindowOptions,
};
use gpui_component::Root;
use gpui_component_assets::Assets;
use gpui_platform::application;
use std::{borrow::Cow, sync::Arc};

actions!(component, [About, Quit]);

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        match path {
            "app-icon.png" => Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/app-icon.png"
            )))),
            "app-icon-source.png" => Ok(Some(Cow::Borrowed(include_bytes!(
                "../assets/app-icon-source.png"
            )))),
            _ => Assets.load(path),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut assets = Assets.list(path)?;
        assets.extend(
            ["app-icon.png", "app-icon-source.png"]
                .into_iter()
                .filter(|asset| asset.starts_with(path))
                .map(SharedString::from),
        );
        Ok(assets)
    }
}

fn window_icon() -> Arc<image::RgbaImage> {
    Arc::new(
        image::load_from_memory(include_bytes!("../assets/app-icon.png"))
            .expect("failed to load application icon")
            .into_rgba8(),
    )
}

fn main() {
    let app = application().with_assets(AppAssets);

    app.with_quit_mode(QuitMode::LastWindowClosed)
        .run(|cx: &mut App| {
            gpui_component::init(cx);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.on_action(|_: &About, cx| {
                if let Some(window) = cx
                    .active_window()
                    .and_then(|window| window.downcast::<Root>())
                {
                    cx.defer(move |cx| {
                        window
                            .update(cx, |_, window, cx| {
                                window.defer(cx, |window, cx| {
                                    open_about_dialog(window, cx);
                                });
                            })
                            .expect("failed to open the About dialog");
                    });
                }
            });
            cx.bind_keys([
                KeyBinding::new("cmd-q", Quit, None),
                KeyBinding::new("ctrl-q", Quit, None),
            ]);
            cx.set_menus([Menu::new("Component Playground").items([
                MenuItem::action("About Component Playground", About),
                MenuItem::separator(),
                MenuItem::action("Quit Component Playground", Quit),
            ])]);

            open_main_window(cx);
            cx.activate(true);
        });
}

fn open_main_window(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(960.), px(640.)), cx);

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(640.), px(420.))),
            window_background: WindowBackgroundAppearance::Blurred,
            app_id: Some("com.component-playground.app".into()),
            icon: Some(window_icon()),
            titlebar: Some(TitlebarOptions {
                title: Some("Component Playground".into()),
                appears_transparent: true,
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            window.set_background_appearance(WindowBackgroundAppearance::Blurred);
            let view = cx.new(|cx| RootView::new(window, cx));
            cx.new(|cx| gpui_component::Root::new(view, window, cx))
        },
    )
    .expect("failed to open Component Playground window");
}
