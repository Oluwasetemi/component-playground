use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::Button, popover::Popover, scroll::ScrollableElement as _, separator::Separator, v_flex,
};

pub struct PopoverStory;

impl PopoverStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for PopoverStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Popover"))
                    .child("Anchored content that appears above the current page."),
            )
            .child(StorySection::new(
                "Popover Content",
                Popover::new("popover-basic")
                    .trigger(
                        Button::new("popover-trigger")
                            .label("Open Popover")
                            .outline(),
                    )
                    .w(px(280.))
                    .child(
                        v_flex()
                            .gap_2()
                            .child("Quick actions")
                            .child(Separator::horizontal())
                            .child("Popovers can contain text, controls, and custom layouts."),
                    ),
            ))
            .child(StorySection::new(
                "Custom Trigger",
                Popover::new("popover-custom")
                    .trigger(
                        Button::new("popover-custom-trigger")
                            .label("Custom trigger")
                            .outline(),
                    )
                    .child("This popover uses a styled div as its trigger."),
            ))
    }
}
