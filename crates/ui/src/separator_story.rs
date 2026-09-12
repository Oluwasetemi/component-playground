use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    h_flex, label::Label, scroll::ScrollableElement as _, separator::Separator, v_flex,
    ActiveTheme as _,
};

pub struct SeparatorStory;

impl SeparatorStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for SeparatorStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Separator"))
                    .child("Horizontal and vertical dividers for organizing content."),
            )
            .child(StorySection::new(
                "Horizontal",
                v_flex()
                    .gap_4()
                    .child(Separator::horizontal())
                    .child(Separator::horizontal().label("With label"))
                    .child(Separator::horizontal_dashed().label("Dashed")),
            ))
            .child(StorySection::new(
                "Vertical",
                h_flex()
                    .items_center()
                    .gap_4()
                    .h(px(80.))
                    .child("Navigation")
                    .child(Separator::vertical())
                    .child("Settings")
                    .child(Separator::vertical_dashed())
                    .child("Help"),
            ))
            .child(StorySection::new(
                "Combined",
                v_flex()
                    .gap_3()
                    .child("Project resources")
                    .child(Separator::horizontal())
                    .child(
                        h_flex()
                            .gap_3()
                            .child(Label::new("Docs").text_color(cx.theme().muted_foreground))
                            .child(Separator::vertical_dashed())
                            .child(Label::new("Source").text_color(cx.theme().muted_foreground))
                            .child(Separator::vertical_dashed())
                            .child(Label::new("Issues").text_color(cx.theme().muted_foreground)),
                    ),
            ))
    }
}
