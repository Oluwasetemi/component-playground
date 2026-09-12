use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    group_box::{GroupBox, GroupBoxVariants as _},
    radio::{Radio, RadioGroup},
    scroll::ScrollableElement as _,
    switch::Switch,
    v_flex, ActiveTheme as _,
};

pub struct GroupBoxStory;

impl GroupBoxStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for GroupBoxStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Group Box"))
                    .child("A styled container for grouping related controls."),
            )
            .child(StorySection::new(
                "Default",
                GroupBox::new()
                    .child("Subscriptions")
                    .child(Checkbox::new("group-all").label("All updates"))
                    .child(Checkbox::new("group-news").label("Newsletters"))
                    .child(
                        Button::new("group-save")
                            .label("Save preferences")
                            .primary(),
                    ),
            ))
            .child(StorySection::new(
                "Fill",
                GroupBox::new()
                    .fill()
                    .title("Activity")
                    .child(
                        Switch::new("group-private")
                            .label("Make profile private")
                            .checked(true),
                    )
                    .child(Switch::new("group-contributions").label("Show contributions")),
            ))
            .child(StorySection::new(
                "Outline",
                GroupBox::new().outline().title("Appearance").child(
                    RadioGroup::vertical("group-theme")
                        .child(Radio::new("light").label("Light"))
                        .child(Radio::new("dark").label("Dark"))
                        .child(Radio::new("system").label("System")),
                ),
            ))
            .child(StorySection::new(
                "Custom Content",
                GroupBox::new().outline().title("About this setting").child(
                    v_flex()
                        .gap_2()
                        .child("Group boxes accept arbitrary child elements.")
                        .child("Use a title to make the relationship clear."),
                ),
            ))
            .text_color(cx.theme().foreground)
    }
}
