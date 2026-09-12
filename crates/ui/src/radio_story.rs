use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    h_flex,
    radio::{Radio, RadioGroup},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct RadioStory {
    first: bool,
    second: bool,
    selected: Option<usize>,
}

impl RadioStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            first: false,
            second: true,
            selected: Some(1),
        }
    }
}

impl Render for RadioStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Radio"))
                    .child("Choose exactly one option from a set."),
            )
            .child(StorySection::new(
                "Individual Radios",
                v_flex()
                    .gap_3()
                    .child(
                        Radio::new("radio-first")
                            .label("First option")
                            .checked(self.first)
                            .on_click(cx.listener(|this, value, _, cx| {
                                this.first = *value;
                                cx.notify();
                            })),
                    )
                    .child(
                        Radio::new("radio-second")
                            .label("Second option")
                            .checked(self.second)
                            .on_click(cx.listener(|this, value, _, cx| {
                                this.second = *value;
                                cx.notify();
                            })),
                    ),
            ))
            .child(StorySection::new(
                "Radio Group",
                RadioGroup::horizontal("radio-group")
                    .children(["One", "Two", "Three"])
                    .selected_index(self.selected)
                    .on_click(cx.listener(|this, selected, _, cx| {
                        this.selected = Some(*selected);
                        cx.notify();
                    })),
            ))
            .child(StorySection::new(
                "Disabled",
                h_flex()
                    .gap_5()
                    .child(
                        Radio::new("radio-disabled")
                            .label("Disabled")
                            .disabled(true),
                    )
                    .child(
                        Radio::new("radio-disabled-checked")
                            .label("Disabled checked")
                            .checked(true)
                            .disabled(true),
                    ),
            ))
    }
}
