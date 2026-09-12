use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonGroup, ButtonVariants as _},
    checkbox::Checkbox,
    h_flex,
    scroll::ScrollableElement as _,
    tab::{Tab, TabBar},
    v_flex, IconName, Selectable as _, Sizable as _, Size,
};

pub struct TabsStory {
    active: usize,
    size: Size,
    menu: bool,
}

impl TabsStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            active: 0,
            size: Size::Medium,
            menu: false,
        }
    }
}

impl Render for TabsStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = |id: &'static str, variant: fn(TabBar) -> TabBar| {
            variant(
                TabBar::new(id)
                    .w_full()
                    .with_size(self.size)
                    .menu(self.menu)
                    .selected_index(self.active)
                    .on_click(cx.listener(|this, index, _, cx| {
                        this.active = *index;
                        cx.notify();
                    }))
                    .child(Tab::new().label("Overview"))
                    .child(Tab::new().label("Activity"))
                    .child(Tab::new().label("Settings"))
                    .child(Tab::new().label("About")),
            )
        };

        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Tabs and Tab Bar"))
                    .child("Layered content navigation with several visual treatments."),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(
                        ButtonGroup::new("tabs-size")
                            .outline()
                            .compact()
                            .child(
                                Button::new("tabs-small")
                                    .label("Small")
                                    .selected(self.size == Size::Small),
                            )
                            .child(
                                Button::new("tabs-medium")
                                    .label("Medium")
                                    .selected(self.size == Size::Medium),
                            )
                            .child(
                                Button::new("tabs-large")
                                    .label("Large")
                                    .selected(self.size == Size::Large),
                            )
                            .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                                if let Some(index) = selected.first() {
                                    this.size = match index {
                                        0 => Size::Small,
                                        1 => Size::Medium,
                                        2 => Size::Large,
                                        _ => return,
                                    };
                                    cx.notify();
                                }
                            })),
                    )
                    .child(
                        Checkbox::new("tabs-menu")
                            .label("More menu")
                            .checked(self.menu)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.menu = *checked;
                                cx.notify();
                            })),
                    ),
            )
            .child(StorySection::new(
                "Default Tabs",
                tabs("tabs-default", |bar| bar),
            ))
            .child(StorySection::new(
                "Underline Tabs",
                tabs("tabs-underline", |bar| bar.underline()),
            ))
            .child(StorySection::new(
                "Pill Tabs",
                tabs("tabs-pill", |bar| bar.pill()),
            ))
            .child(StorySection::new(
                "Outline Tabs",
                tabs("tabs-outline", |bar| bar.outline()),
            ))
            .child(StorySection::new(
                "Segmented Tabs",
                TabBar::new("tabs-segmented")
                    .w_full()
                    .segmented()
                    .with_size(self.size)
                    .selected_index(self.active)
                    .on_click(cx.listener(|this, index, _, cx| {
                        this.active = *index;
                        cx.notify();
                    }))
                    .prefix(
                        Button::new("tabs-back")
                            .ghost()
                            .xsmall()
                            .icon(IconName::ArrowLeft),
                    )
                    .child("Files")
                    .child("Search")
                    .child("Preview")
                    .suffix(
                        Button::new("tabs-more")
                            .ghost()
                            .xsmall()
                            .icon(IconName::Ellipsis),
                    ),
            ))
    }
}
