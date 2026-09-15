use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    notification::{Notification, NotificationType},
    scroll::ScrollableElement as _,
    v_flex, WindowExt as _,
};

pub struct NotificationStory;

impl NotificationStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for NotificationStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Notification"))
                    .child("Transient messages for completed actions and system status."),
            )
            .child(StorySection::new(
                "Notification Types",
                gpui_component::h_flex()
                    .flex_wrap()
                    .gap_3()
                    .child(
                        Button::new("notify-simple")
                            .label("Simple")
                            .outline()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification("This is a notification.", cx);
                            })),
                    )
                    .child(
                        Button::new("notify-info")
                            .label("Info")
                            .info()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    (
                                        NotificationType::Info,
                                        "An informational update is available.",
                                    ),
                                    cx,
                                );
                            })),
                    )
                    .child(
                        Button::new("notify-success")
                            .label("Success")
                            .success()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    Notification::success("Your changes were saved.")
                                        .title("All changes saved"),
                                    cx,
                                );
                            })),
                    )
                    .child(
                        Button::new("notify-error")
                            .label("Error")
                            .danger()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    (
                                        NotificationType::Error,
                                        "The request could not be completed.",
                                    ),
                                    cx,
                                );
                            })),
                    ),
            ))
    }
}
