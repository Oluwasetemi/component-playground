use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{scroll::ScrollableElement as _, skeleton::Skeleton, v_flex};

pub struct SkeletonStory;

impl SkeletonStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for SkeletonStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Skeleton"))
                    .child("Placeholder shapes for content that is still loading."),
            )
            .child(StorySection::new(
                "Basic Shapes",
                v_flex()
                    .gap_3()
                    .child(Skeleton::new().size_12().rounded_full())
                    .child(Skeleton::new().w(px(280.)).h_4())
                    .child(Skeleton::new().w(px(220.)).h_4())
                    .child(Skeleton::new().w(px(160.)).h_4()),
            ))
            .child(StorySection::new(
                "Card Placeholder",
                v_flex()
                    .gap_3()
                    .child(Skeleton::new().w(px(320.)).h(px(140.)).rounded(px(8.)))
                    .child(Skeleton::new().w(px(260.)).h_4())
                    .child(Skeleton::new().w(px(200.)).h_4()),
            ))
    }
}
