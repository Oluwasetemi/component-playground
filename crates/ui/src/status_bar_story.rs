use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::ScrollableElement as _,
    separator::Separator,
    status_bar::StatusBar,
    v_flex, Icon, IconName, Sizable as _,
};

pub struct StatusBarStory;

impl StatusBarStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for StatusBarStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Status Bar"))
                    .child("A compact horizontal bar with left, center, and right regions."),
            )
            .child(StorySection::new(
                "Editor Status",
                StatusBar::new()
                    .left(
                        Button::new("status-branch")
                            .ghost()
                            .xsmall()
                            .icon(IconName::Github)
                            .label("main"),
                    )
                    .left(Separator::vertical().h_3())
                    .left(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::CircleCheck))
                            .child("0 errors"),
                    )
                    .child("Ready")
                    .right("Ln 12, Col 34")
                    .right(Separator::vertical().h_3())
                    .right("UTF-8"),
            ))
            .child(StorySection::new(
                "Layout",
                v_flex()
                    .gap_3()
                    .child(StatusBar::new().child("Center only"))
                    .child(StatusBar::new().left("Left").child("Center").right("Right"))
                    .child(
                        StatusBar::new()
                            .left("Connected")
                            .right("All changes saved"),
                    ),
            ))
    }
}
