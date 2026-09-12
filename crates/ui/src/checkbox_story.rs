use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    checkbox::Checkbox, h_flex, scroll::ScrollableElement as _, v_flex, Disableable as _,
    Sizable as _,
};

pub struct CheckboxStory {
    checked: [bool; 3],
}

impl CheckboxStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            checked: [false, true, false],
        }
    }
}

impl Render for CheckboxStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Checkbox"))
                    .child("A control that can be checked or unchecked."),
            )
            .child(StorySection::new(
                "Interactive Checkboxes",
                v_flex()
                    .gap_3()
                    .child(
                        Checkbox::new("checkbox-normal")
                            .label("Email notifications")
                            .checked(self.checked[0])
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.checked[0] = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("checkbox-remember")
                            .label("Remember my choice")
                            .checked(self.checked[1])
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.checked[1] = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("checkbox-small")
                            .small()
                            .label("Small checkbox")
                            .checked(self.checked[2])
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.checked[2] = *checked;
                                cx.notify();
                            })),
                    ),
            ))
            .child(StorySection::new(
                "Disabled",
                h_flex()
                    .gap_6()
                    .child(
                        Checkbox::new("checkbox-disabled")
                            .label("Disabled")
                            .disabled(true),
                    )
                    .child(
                        Checkbox::new("checkbox-disabled-checked")
                            .label("Disabled checked")
                            .checked(true)
                            .disabled(true),
                    ),
            ))
            .child(StorySection::new(
                "Long Description",
                Checkbox::new("checkbox-description")
                    .w(px(320.))
                    .label("Include activity in the weekly summary")
                    .child(div().child("A longer description can explain this preference.")),
            ))
    }
}
