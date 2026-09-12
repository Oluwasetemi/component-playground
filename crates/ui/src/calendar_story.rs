use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    calendar::{Calendar, CalendarState},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct CalendarStory {
    calendar: gpui::Entity<CalendarState>,
    months: gpui::Entity<CalendarState>,
}

impl CalendarStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            calendar: cx.new(|cx| CalendarState::new(window, cx)),
            months: cx.new(|cx| CalendarState::new(window, cx)),
        }
    }
}

impl Render for CalendarStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Calendar"))
                    .child("Inline calendar views for date navigation and selection."),
            )
            .child(StorySection::new(
                "Single Month",
                Calendar::new(&self.calendar),
            ))
            .child(StorySection::new(
                "Multiple Months",
                Calendar::new(&self.months).number_of_months(2),
            ))
    }
}
