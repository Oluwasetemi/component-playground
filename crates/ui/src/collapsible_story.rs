use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::Button, collapsible::Collapsible, h_flex, scroll::ScrollableElement as _, v_flex,
    IconName, Sizable as _,
};

pub struct CollapsibleStory {
    open: bool,
}

impl CollapsibleStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { open: false }
    }
}

impl Render for CollapsibleStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Collapsible"))
                    .child("Progressively disclose optional content."),
            )
            .child(StorySection::new(
                "Expandable Content",
                Collapsible::new()
                    .gap_2()
                    .open(self.open)
                    .child(
                        h_flex().justify_between().child("Project details").child(
                            Button::new("collapsible-toggle")
                                .label("Details")
                                .small()
                                .outline()
                                .icon(if self.open {
                                    IconName::ChevronUp
                                } else {
                                    IconName::ChevronDown
                                })
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.open = !this.open;
                                    cx.notify();
                                })),
                        ),
                    )
                    .content(
                        v_flex()
                            .gap_2()
                            .child("This content is rendered only while the section is open.")
                            .child("It can contain any GPUI element, including buttons and forms."),
                    ),
            ))
            .child(StorySection::new(
                "Always Visible Header",
                Collapsible::new()
                    .open(true)
                    .child("This header remains visible when the content is collapsed.")
                    .content("Additional context appears here when expanded."),
            ))
    }
}
