use gpui::{div, prelude::*, App, IntoElement, RenderOnce, Window};
use gpui_component::button::ButtonVariants;
use gpui_component::{button::Button as ComponentButton, StyledExt};

#[derive(IntoElement)]
pub struct HelloWorld;

impl RenderOnce for HelloWorld {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .items_center()
            .justify_center()
            .child("Longbridge GPUI Component")
            .child(
                ComponentButton::new("longbridge-smoke-test")
                    .primary()
                    .label("Longbridge Button")
                    .on_click(|_, _, _| println!("Longbridge button clicked")),
            )
    }
}
