use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    scroll::ScrollableElement as _,
    v_flex, WindowExt as _,
};

pub struct AlertDialogStory;

impl AlertDialogStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for AlertDialogStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(v_flex().gap_1().child(div().text_size(px(24.)).child("Alert Dialog")).child("Interruptive confirmation for destructive or important actions."))
            .child(StorySection::new(
                "Confirmations",
                v_flex()
                    .gap_3()
                    .child(Button::new("alert-info").label("Show Information").outline().on_click(cx.listener(|_, _, window, cx| {
                        window.open_alert_dialog(cx, |alert, _, _| alert.title("Information").description("Your export is ready to download."));
                    })))
                    .child(Button::new("alert-delete").label("Delete Account").danger().on_click(cx.listener(|_, _, window, cx| {
                        window.open_alert_dialog(cx, |alert, _, _| alert.title("Delete account?").description("This permanently removes your account and all associated data."));
                    })))
            ))
    }
}
