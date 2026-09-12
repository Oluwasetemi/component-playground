use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    scroll::ScrollableElement as _,
    slider::{Slider, SliderScale, SliderState},
    v_flex,
};

pub struct SliderStory {
    volume: gpui::Entity<SliderState>,
    range: gpui::Entity<SliderState>,
    logarithmic: gpui::Entity<SliderState>,
}

impl SliderStory {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            volume: cx.new(|_| {
                SliderState::new()
                    .min(0.)
                    .max(100.)
                    .step(5.)
                    .default_value(65.)
            }),
            range: cx.new(|_| {
                SliderState::new()
                    .min(0.)
                    .max(100.)
                    .step(1.)
                    .default_value(25. ..75.)
            }),
            logarithmic: cx.new(|_| {
                SliderState::new()
                    .min(0.25)
                    .max(4.)
                    .step(0.05)
                    .default_value(1.)
                    .scale(SliderScale::Logarithmic)
            }),
        }
    }
}

impl Render for SliderStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Slider"))
                    .child("Continuous controls for selecting scalar and range values."),
            )
            .child(StorySection::new(
                "Single Value",
                Slider::new(&self.volume).w(px(420.)),
            ))
            .child(StorySection::new(
                "Range",
                Slider::new(&self.range).w(px(420.)),
            ))
            .child(StorySection::new(
                "Logarithmic",
                Slider::new(&self.logarithmic).w(px(420.)),
            ))
    }
}
