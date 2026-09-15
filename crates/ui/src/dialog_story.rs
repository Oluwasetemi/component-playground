use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    dialog::{
        DialogAction, DialogClose, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    },
    scroll::ScrollableElement as _,
    v_flex, WindowExt as _,
};

pub struct DialogStory;

impl DialogStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for DialogStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Dialog"))
                    .child("Modal content for focused tasks and workflows."),
            )
            .child(StorySection::new(
                "Dialog Variants",
                v_flex()
                    .gap_3()
                    .child(
                        Button::new("dialog-basic")
                            .label("Open Dialog")
                            .primary()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.open_dialog(cx, |dialog, _, _| {
                                    dialog
                                        .title("Project details")
                                        .child("This is a standard dialog with arbitrary content.")
                                });
                            })),
                    )
                    .child(
                        Button::new("dialog-composed")
                            .label("Open Composed Dialog")
                            .outline()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.open_dialog(cx, |dialog, _, _| {
                                    dialog.content(|content, _, _| {
                                        content
                                            .child(
                                                DialogHeader::new()
                                                    .child(
                                                        DialogTitle::new().child("Delete project"),
                                                    )
                                                    .child(
                                                        DialogDescription::new()
                                                            .child("This action cannot be undone."),
                                                    ),
                                            )
                                            .child(div().px_4().pb_4().child(
                                                "All project data and deployments will be removed.",
                                            ))
                                            .child(
                                                DialogFooter::new()
                                                    .child(
                                                        DialogClose::new().child(
                                                            Button::new("dialog-cancel")
                                                                .label("Cancel")
                                                                .outline(),
                                                        ),
                                                    )
                                                    .child(
                                                        DialogAction::new().child(
                                                            Button::new("dialog-delete")
                                                                .label("Delete")
                                                                .danger(),
                                                        ),
                                                    ),
                                            )
                                    })
                                });
                            })),
                    ),
            ))
    }
}
