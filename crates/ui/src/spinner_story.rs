use crate::StorySection;
use gpui::{div, ease_in_out, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    h_flex, scroll::ScrollableElement as _, spinner::Spinner, v_flex, ActiveTheme as _, IconName,
    Sizable as _,
};

pub struct SpinnerStory;

impl SpinnerStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for SpinnerStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Spinner"))
                    .child("Animated indicators for work in progress."),
            )
            .child(StorySection::new(
                "Sizes",
                h_flex()
                    .items_center()
                    .gap_4()
                    .child(Spinner::new().with_size(px(48.)))
                    .child(Spinner::new().large())
                    .child(Spinner::new())
                    .child(Spinner::new().small())
                    .child(Spinner::new().xsmall()),
            ))
            .child(StorySection::new(
                "Colors and Icons",
                h_flex()
                    .items_center()
                    .gap_4()
                    .child(Spinner::new().color(cx.theme().blue))
                    .child(Spinner::new().color(cx.theme().green))
                    .child(
                        Spinner::new()
                            .icon(IconName::LoaderCircle)
                            .color(cx.theme().cyan),
                    )
                    .child(Spinner::new().icon(IconName::Loader).ease(ease_in_out)),
            ))
    }
}
