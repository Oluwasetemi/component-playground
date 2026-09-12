use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    input::{InputState, NumberInput},
    scroll::ScrollableElement as _,
    v_flex, Disableable as _, Sizable as _,
};

pub struct NumberInputStory {
    integer: gpui::Entity<InputState>,
    decimal: gpui::Entity<InputState>,
    disabled: gpui::Entity<InputState>,
}

impl NumberInputStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            integer: cx.new(|cx| InputState::new(window, cx).default_value("12").min(0.)),
            decimal: cx.new(|cx| {
                InputState::new(window, cx)
                    .default_value("1234.50")
                    .step(0.5)
                    .min(0.)
            }),
            disabled: cx.new(|cx| InputState::new(window, cx).default_value("100")),
        }
    }
}

impl Render for NumberInputStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Number Input"))
                    .child("Numeric fields with increment and decrement controls."),
            )
            .child(StorySection::new(
                "Integer",
                NumberInput::new(&self.integer).w(px(220.)),
            ))
            .child(StorySection::new(
                "Decimal Step",
                NumberInput::new(&self.decimal).small().w(px(220.)),
            ))
            .child(StorySection::new(
                "Disabled",
                NumberInput::new(&self.disabled).disabled(true),
            ))
    }
}
