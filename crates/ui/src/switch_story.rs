use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    h_flex, label::Label, scroll::ScrollableElement as _, switch::Switch, v_flex, ActiveTheme as _,
    Disableable as _, Sizable as _,
};

pub struct SwitchStory {
    marketing: bool,
    security: bool,
    success: bool,
}

impl SwitchStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            marketing: true,
            security: false,
            success: true,
        }
    }
}

impl Render for SwitchStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Switch"))
                    .child("Toggle a setting between enabled and disabled."),
            )
            .child(StorySection::new(
                "Settings",
                v_flex()
                    .gap_3()
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                v_flex().gap_1().child("Marketing emails").child(
                                    Label::new("Product news and tips")
                                        .text_color(cx.theme().muted_foreground),
                                ),
                            )
                            .child(
                                Switch::new("switch-marketing")
                                    .checked(self.marketing)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.marketing = *checked;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .justify_between()
                            .child(
                                v_flex().gap_1().child("Security emails").child(
                                    Label::new("Important account notices")
                                        .text_color(cx.theme().muted_foreground),
                                ),
                            )
                            .child(
                                Switch::new("switch-security")
                                    .checked(self.security)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.security = *checked;
                                        cx.notify();
                                    })),
                            ),
                    ),
            ))
            .child(StorySection::new(
                "Variants",
                h_flex()
                    .items_center()
                    .gap_4()
                    .child(
                        Switch::new("switch-success")
                            .label("Success")
                            .checked(self.success)
                            .color(cx.theme().green)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.success = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Switch::new("switch-danger")
                            .label("Destructive")
                            .checked(true)
                            .color(cx.theme().red),
                    )
                    .child(
                        Switch::new("switch-disabled")
                            .label("Disabled")
                            .checked(true)
                            .disabled(true),
                    ),
            ))
            .child(StorySection::new(
                "Small",
                Switch::new("switch-small")
                    .small()
                    .label("Compact setting")
                    .checked(true),
            ))
    }
}
