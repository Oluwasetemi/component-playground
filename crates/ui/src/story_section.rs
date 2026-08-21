use gpui::{div, prelude::*, px, AnyElement, App, IntoElement, RenderOnce, SharedString, Window};
use gpui_component::v_flex;

#[derive(IntoElement)]
pub struct StorySection {
    title: SharedString,
    content: AnyElement,
}

impl StorySection {
    pub fn new(title: impl Into<SharedString>, content: impl IntoElement) -> Self {
        Self {
            title: title.into(),
            content: content.into_any_element(),
        }
    }
}

impl RenderOnce for StorySection {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(div().text_size(px(16.)).child(self.title))
            .child(self.content)
    }
}
