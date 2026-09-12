use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    form::{field, v_form},
    input::{Input, InputState},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct FormStory {
    name: gpui::Entity<InputState>,
    email: gpui::Entity<InputState>,
    bio: gpui::Entity<InputState>,
}

impl FormStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            name: cx.new(|cx| InputState::new(window, cx).placeholder("Your name")),
            email: cx.new(|cx| InputState::new(window, cx).placeholder("you@example.com")),
            bio: cx.new(|cx| {
                InputState::new(window, cx)
                    .multi_line(true)
                    .auto_grow(3, 6)
                    .placeholder("Tell us about yourself")
            }),
        }
    }
}

impl Render for FormStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Form"))
                    .child("Compose labeled, validated fields into a consistent layout."),
            )
            .child(StorySection::new(
                "Vertical Form",
                v_form()
                    .w(px(520.))
                    .child(field().label("Name").child(Input::new(&self.name)))
                    .child(
                        field()
                            .label("Email")
                            .required(true)
                            .child(Input::new(&self.email)),
                    )
                    .child(
                        field()
                            .label("Bio")
                            .description("A short description helps personalize your profile.")
                            .child(Input::new(&self.bio)),
                    ),
            ))
    }
}
