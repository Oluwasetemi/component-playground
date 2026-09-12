use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    input::{OtpInput, OtpState},
    scroll::ScrollableElement as _,
    v_flex, Disableable as _, Sizable as _,
};

pub struct OtpInputStory {
    normal: gpui::Entity<OtpState>,
    compact: gpui::Entity<OtpState>,
    disabled: gpui::Entity<OtpState>,
}

impl OtpInputStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            normal: cx.new(|cx| OtpState::new(6, window, cx)),
            compact: cx.new(|cx| OtpState::new(4, window, cx).default_value("1234")),
            disabled: cx.new(|cx| OtpState::new(6, window, cx).default_value("123456")),
        }
    }
}

impl Render for OtpInputStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("OTP Input"))
                    .child("Fixed-length one-time password fields with grouped cells."),
            )
            .child(StorySection::new("Default", OtpInput::new(&self.normal)))
            .child(StorySection::new(
                "Small and Grouped",
                OtpInput::new(&self.compact).groups(1).small(),
            ))
            .child(StorySection::new(
                "Disabled",
                OtpInput::new(&self.disabled)
                    .disabled(true)
                    .with_size(px(42.)),
            ))
    }
}
