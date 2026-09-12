use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    date_picker::{DatePicker, DatePickerState},
    scroll::ScrollableElement as _,
    v_flex, Sizable as _,
};

pub struct DatePickerStory {
    date: gpui::Entity<DatePickerState>,
    range: gpui::Entity<DatePickerState>,
}

impl DatePickerStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            date: cx.new(|cx| DatePickerState::new(window, cx)),
            range: cx.new(|cx| DatePickerState::range(window, cx)),
        }
    }
}

impl Render for DatePickerStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Date Picker"))
                    .child("Pick a date or date range from a calendar popup."),
            )
            .child(StorySection::new(
                "Single Date",
                DatePicker::new(&self.date).cleanable(true).w(px(280.)),
            ))
            .child(StorySection::new(
                "Date Range",
                DatePicker::new(&self.range)
                    .placeholder("Select a date range")
                    .small()
                    .w(px(280.)),
            ))
    }
}
