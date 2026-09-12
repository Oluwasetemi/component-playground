use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    input::{Input, InputContentType, InputState},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct InputStory {
    normal: gpui::Entity<InputState>,
    password: gpui::Entity<InputState>,
    semantic: gpui::Entity<InputState>,
    disabled: gpui::Entity<InputState>,
}

impl InputStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            normal: cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name")),
            password: cx.new(|cx| {
                InputState::new(window, cx)
                    .default_value("correct horse battery staple")
                    .masked(true)
            }),
            semantic: cx.new(|cx| InputState::new(window, cx).placeholder("you@example.com")),
            disabled: cx.new(|cx| InputState::new(window, cx).default_value("Read only value")),
        }
    }
}

impl Render for InputStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Input"))
                    .child("Single-line text fields with masking, prefixes, and semantic hints."),
            )
            .child(StorySection::new(
                "Text Input",
                Input::new(&self.normal).prefix("Name"),
            ))
            .child(StorySection::new(
                "Password",
                Input::new(&self.password).mask_toggle(),
            ))
            .child(StorySection::new(
                "Prefix and Sizing",
                Input::new(&self.semantic)
                    .content_type(InputContentType::EmailAddress)
                    .prefix("Email")
                    .suffix("Verified")
                    .w(px(360.)),
            ))
            .child(StorySection::new(
                "Disabled",
                Input::new(&self.disabled).disabled(true),
            ))
    }
}
