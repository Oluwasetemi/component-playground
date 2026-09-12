use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::Button, h_flex, rating::Rating, scroll::ScrollableElement as _, v_flex,
    ActiveTheme as _, Sizable as _,
};

pub struct RatingStory {
    value: usize,
}

impl RatingStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { value: 3 }
    }
}

impl Render for RatingStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Rating"))
                    .child("Interactive star ratings with custom values and colors."),
            )
            .child(StorySection::new(
                "Interactive Rating",
                v_flex()
                    .gap_3()
                    .child(
                        Rating::new("rating-main")
                            .value(self.value)
                            .max(5)
                            .on_click(cx.listener(|this, value, _, cx| {
                                this.value = *value;
                                cx.notify();
                            })),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("rating-down")
                                    .small()
                                    .outline()
                                    .label("Less")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.value = this.value.saturating_sub(1);
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Button::new("rating-up")
                                    .small()
                                    .outline()
                                    .label("More")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.value = (this.value + 1).min(5);
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(format!("{} of 5", self.value)),
            ))
            .child(StorySection::new(
                "Variants",
                h_flex()
                    .items_center()
                    .gap_4()
                    .child(Rating::new("rating-disabled").value(2).disabled(true))
                    .child(
                        Rating::new("rating-green")
                            .large()
                            .value(4)
                            .color(cx.theme().green),
                    ),
            ))
    }
}
