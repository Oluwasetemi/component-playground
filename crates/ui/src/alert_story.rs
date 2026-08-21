use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    alert::Alert,
    button::{Button, ButtonGroup},
    scroll::ScrollableElement as _,
    v_flex, Selectable as _, Sizable as _, Size,
};

pub struct AlertStory {
    size: Size,
    banner_visible: bool,
}

impl AlertStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            size: Size::Medium,
            banner_visible: true,
        }
    }

    fn controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        ButtonGroup::new("alert-size")
            .outline()
            .compact()
            .child(
                Button::new("alert-size-xsmall")
                    .label("XSmall")
                    .selected(self.size == Size::XSmall),
            )
            .child(
                Button::new("alert-size-small")
                    .label("Small")
                    .selected(self.size == Size::Small),
            )
            .child(
                Button::new("alert-size-medium")
                    .label("Medium")
                    .selected(self.size == Size::Medium),
            )
            .child(
                Button::new("alert-size-large")
                    .label("Large")
                    .selected(self.size == Size::Large),
            )
            .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                if let Some(index) = selected.first() {
                    this.size = match index {
                        0 => Size::XSmall,
                        1 => Size::Small,
                        2 => Size::Medium,
                        3 => Size::Large,
                        _ => return,
                    };
                    cx.notify();
                }
            }))
    }

    fn variants(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StorySection::new(
            "Variants",
            v_flex()
                .gap_2()
                .child(
                    Alert::info("alert-info", "This is an informational alert.")
                        .with_size(self.size)
                        .title("Information"),
                )
                .child(
                    Alert::success("alert-success", "Your changes have been saved.")
                        .with_size(self.size)
                        .title("Success"),
                )
                .child(
                    Alert::warning("alert-warning", "Review this setting before continuing.")
                        .with_size(self.size)
                        .title("Warning"),
                )
                .child(
                    Alert::error("alert-error", "We could not complete that operation.")
                        .with_size(self.size)
                        .title("Error")
                        .on_close(cx.listener(|_, _, _, _| {
                            println!("Error alert closed");
                        })),
                ),
        )
    }

    fn banners(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StorySection::new(
            "Banners",
            v_flex()
                .gap_2()
                .child(
                    Alert::new(
                        "alert-banner-default",
                        "This alert spans the available width as a banner.",
                    )
                    .banner()
                    .visible(self.banner_visible)
                    .with_size(self.size)
                    .on_close(cx.listener(|this, _, _, cx| {
                        this.banner_visible = false;
                        cx.notify();
                    })),
                )
                .child(
                    Alert::success(
                        "alert-banner-success",
                        "Banner alerts are useful for page-level status messages.",
                    )
                    .banner()
                    .with_size(self.size),
                ),
        )
    }
}

impl Render for AlertStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Alert"))
                    .child("Callouts for important status and attention messages."),
            )
            .child(self.controls(cx))
            .child(self.variants(cx))
            .child(self.banners(cx))
            .child(StorySection::new(
                "Custom Icon",
                Alert::new("alert-custom-icon", "This alert uses a calendar icon.")
                    .title("Custom alert")
                    .with_size(self.size)
                    .icon(gpui_component::IconName::Calendar),
            ))
    }
}
