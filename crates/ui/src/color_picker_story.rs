use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    color_picker::{ColorPicker, ColorPickerState},
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme as _, Sizable as _,
};

pub struct ColorPickerStory {
    color: gpui::Entity<ColorPickerState>,
}

impl ColorPickerStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            color: cx.new(|cx| ColorPickerState::new(window, cx).default_value(cx.theme().primary)),
        }
    }
}

impl Render for ColorPickerStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Color Picker"))
                    .child("Select a color from a compact trigger and palette."),
            )
            .child(StorySection::new(
                "Picker",
                ColorPicker::new(&self.color).small().w(px(280.)),
            ))
    }
}
