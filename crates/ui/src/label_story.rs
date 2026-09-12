use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme as _, StyledExt as _,
};

pub struct LabelStory {
    masked: bool,
}

impl LabelStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { masked: false }
    }
}

impl Render for LabelStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Label"))
                    .child("Text labels with secondary content and styling."),
            )
            .child(StorySection::new(
                "Labels",
                v_flex()
                    .gap_3()
                    .child(Label::new("Workspace name"))
                    .child(Label::new("Company address").secondary("(optional)"))
                    .child(Label::new("Large heading").text_2xl().font_semibold())
                    .child(Label::new("Muted helper text").text_color(cx.theme().muted_foreground)),
            ))
            .child(StorySection::new(
                "Alignment and Wrapping",
                v_flex()
                    .gap_3()
                    .child(Label::new("Centered label").w(px(280.)).text_center())
                    .child(
                        Label::new("A long label wraps cleanly when its container is narrow.")
                            .w(px(280.)),
                    ),
            ))
            .child(StorySection::new(
                "Masked Value",
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(Label::new("9,182.10 USD").text_xl().masked(self.masked))
                    .child(
                        Button::new("label-mask")
                            .ghost()
                            .icon(if self.masked {
                                gpui_component::IconName::Eye
                            } else {
                                gpui_component::IconName::EyeOff
                            })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.masked = !this.masked;
                                cx.notify();
                            })),
                    ),
            ))
    }
}
