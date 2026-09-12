use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Keystroke, Render, Window};
use gpui_component::{h_flex, kbd::Kbd, scroll::ScrollableElement as _, v_flex};

pub struct KbdStory;

impl KbdStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for KbdStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Kbd"))
                    .child("Compact keyboard shortcut labels."),
            )
            .child(StorySection::new(
                "Keyboard Shortcuts",
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Kbd::new(Keystroke::parse("cmd-shift-p").unwrap()))
                    .child(Kbd::new(Keystroke::parse("cmd-k").unwrap()))
                    .child(Kbd::new(Keystroke::parse("escape").unwrap()))
                    .child(Kbd::new(Keystroke::parse("enter").unwrap())),
            ))
            .child(StorySection::new(
                "Outline",
                h_flex()
                    .gap_2()
                    .child(Kbd::new(Keystroke::parse("cmd-s").unwrap()).outline())
                    .child(Kbd::new(Keystroke::parse("cmd-z").unwrap()).outline())
                    .child(Kbd::new(Keystroke::parse("backspace").unwrap()).outline()),
            ))
    }
}
