use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{h_flex, link::Link, scroll::ScrollableElement as _, v_flex};

pub struct LinkStory {
    clicked: bool,
}

impl LinkStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { clicked: false }
    }
}

impl Render for LinkStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Link"))
                    .child("Text links for navigation and actions."),
            )
            .child(StorySection::new(
                "Links",
                h_flex()
                    .gap_4()
                    .child(
                        Link::new("link-docs")
                            .href("https://gpui-kit.com")
                            .child("Documentation"),
                    )
                    .child(
                        Link::new("link-source")
                            .href("https://github.com/longbridge/gpui-kit")
                            .child("Source code"),
                    )
                    .child(Link::new("link-disabled").disabled(true).child("Disabled")),
            ))
            .child(StorySection::new(
                "Interactive Link",
                v_flex()
                    .gap_2()
                    .child(
                        Link::new("link-action")
                            .child("Click to select this link")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.clicked = true;
                                cx.notify();
                            })),
                    )
                    .when(self.clicked, |this| this.child("Link selected")),
            ))
    }
}
