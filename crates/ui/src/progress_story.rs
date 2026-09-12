use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::Button,
    h_flex,
    progress::{Progress, ProgressCircle},
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme as _, Selectable as _, Sizable as _,
};

pub struct ProgressStory {
    value: f32,
    loading: bool,
}

impl ProgressStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            value: 42.,
            loading: false,
        }
    }
}

impl Render for ProgressStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Progress"))
                    .child("Indicators for determinate and indeterminate work."),
            )
            .child(StorySection::new(
                "Controls",
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("progress-0")
                            .small()
                            .label("0%")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.value = 0.;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("progress-42")
                            .small()
                            .label("42%")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.value = 42.;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("progress-100")
                            .small()
                            .label("100%")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.value = 100.;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("progress-loading")
                            .small()
                            .label("Loading")
                            .selected(self.loading)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.loading = !this.loading;
                                cx.notify();
                            })),
                    ),
            ))
            .child(StorySection::new(
                "Progress Bar",
                Progress::new("progress-bar")
                    .value(self.value)
                    .loading(self.loading),
            ))
            .child(StorySection::new(
                "Custom Bar",
                Progress::new("progress-custom")
                    .value(68.)
                    .h(px(12.))
                    .rounded(px(6.))
                    .color(cx.theme().green),
            ))
            .child(StorySection::new(
                "Circle Progress",
                h_flex()
                    .items_center()
                    .gap_4()
                    .child(
                        ProgressCircle::new("progress-circle")
                            .value(self.value)
                            .loading(self.loading)
                            .size_16(),
                    )
                    .child(
                        ProgressCircle::new("progress-circle-small")
                            .value(75.)
                            .color(cx.theme().cyan)
                            .size_8(),
                    )
                    .child(if self.loading { "Working..." } else { "Ready" }),
            ))
            .child(
                div()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!("Current value: {}%", self.value)),
            )
    }
}
