use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    scroll::ScrollableElement as _,
    switch::Switch,
    v_flex,
};

pub struct TooltipStory;

impl TooltipStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for TooltipStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Tooltip"))
                    .child("Short contextual hints shown on hover or focus."),
            )
            .child(StorySection::new(
                "Interactive Controls",
                v_flex()
                    .gap_3()
                    .child(
                        Button::new("tooltip-primary")
                            .label("Hover for help")
                            .primary()
                            .tooltip("This action saves your changes."),
                    )
                    .child(
                        Checkbox::new("tooltip-checkbox")
                            .label("Remember me")
                            .checked(true)
                            .tooltip("Keep this preference for your next visit."),
                    )
                    .child(
                        Switch::new("tooltip-switch")
                            .label("Sync automatically")
                            .checked(true)
                            .tooltip("Sync changes across your devices."),
                    ),
            ))
    }
}
