use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    scroll::ScrollableElement as _,
    v_flex, Placement, WindowExt as _,
};

pub struct SheetStory;

impl SheetStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for SheetStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Sheet"))
                    .child("Edge-anchored panels for secondary workflows."),
            )
            .child(StorySection::new(
                "Placements",
                gpui_component::h_flex()
                    .flex_wrap()
                    .gap_3()
                    .child(
                        Button::new("sheet-left")
                            .label("Left")
                            .outline()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
                                    sheet
                                        .title("Left Sheet")
                                        .child("This sheet opens from the left edge.")
                                });
                            })),
                    )
                    .child(
                        Button::new("sheet-right")
                            .label("Right")
                            .primary()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.open_sheet_at(Placement::Right, cx, |sheet, _, _| {
                                    sheet
                                        .title("Right Sheet")
                                        .child("This sheet opens from the right edge.")
                                });
                            })),
                    )
                    .child(
                        Button::new("sheet-bottom")
                            .label("Bottom")
                            .outline()
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.open_sheet_at(Placement::Bottom, cx, |sheet, _, _| {
                                    sheet
                                        .title("Bottom Sheet")
                                        .child("This sheet opens from the bottom edge.")
                                });
                            })),
                    ),
            ))
    }
}
