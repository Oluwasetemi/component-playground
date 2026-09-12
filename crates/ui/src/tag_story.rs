use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{h_flex, scroll::ScrollableElement as _, tag::Tag, v_flex, Sizable as _};

pub struct TagStory;

impl TagStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for TagStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Tag"))
                    .child("Short labels for categorizing or highlighting content."),
            )
            .child(StorySection::new(
                "Variants",
                h_flex()
                    .flex_wrap()
                    .gap_2()
                    .child(Tag::primary().child("Primary"))
                    .child(Tag::secondary().child("Secondary"))
                    .child(Tag::success().child("Success"))
                    .child(Tag::warning().child("Warning"))
                    .child(Tag::danger().child("Danger"))
                    .child(Tag::info().child("Info")),
            ))
            .child(StorySection::new(
                "Outline and Size",
                h_flex()
                    .flex_wrap()
                    .gap_2()
                    .child(Tag::primary().outline().child("Outline"))
                    .child(Tag::success().small().child("Small"))
                    .child(Tag::info().rounded_full().child("Rounded"))
                    .child(Tag::danger().small().rounded_full().child("Compact danger")),
            ))
            .child(StorySection::new(
                "Status Labels",
                v_flex()
                    .gap_2()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Tag::success().child("Published"))
                            .child("Release is live"),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Tag::warning().child("Review"))
                            .child("Awaiting approval"),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Tag::danger().child("Blocked"))
                            .child("Action required"),
                    ),
            ))
    }
}
