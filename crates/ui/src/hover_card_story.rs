use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    avatar::Avatar, button::Button, hover_card::HoverCard, scroll::ScrollableElement as _, v_flex,
    ActiveTheme, Sizable,
};

pub struct HoverCardStory;

impl HoverCardStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for HoverCardStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Hover Card"))
                    .child("Rich previews revealed without navigating away."),
            )
            .child(StorySection::new(
                "Profile Preview",
                HoverCard::new("hover-profile")
                    .trigger(
                        Button::new("hover-trigger")
                            .label("Hover over profile")
                            .outline(),
                    )
                    .child(
                        gpui_component::h_flex()
                            .gap_3()
                            .items_start()
                            .child(Avatar::new().name("Jane Doe").large())
                            .child(
                                v_flex().gap_1().child("Jane Doe").child(
                                    div()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Product designer"),
                                ),
                            ),
                    ),
            ))
            .child(StorySection::new(
                "Delayed Preview",
                HoverCard::new("hover-delay")
                    .open_delay(std::time::Duration::from_millis(400))
                    .trigger(
                        div()
                            .text_color(cx.theme().primary)
                            .child("Hover for project details"),
                    )
                    .child("This preview uses a custom open delay."),
            ))
    }
}
