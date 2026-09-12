use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    scroll::ScrollableElement as _,
    v_flex,
};

pub struct BreadcrumbStory {
    clicked_item: Option<String>,
}

impl BreadcrumbStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { clicked_item: None }
    }
}

impl Render for BreadcrumbStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Breadcrumb"))
                    .child("Hierarchical navigation for showing the current location."),
            )
            .child(StorySection::new(
                "Basic Breadcrumb",
                Breadcrumb::new()
                    .child("Home")
                    .child("Documents")
                    .child("Projects"),
            ))
            .child(StorySection::new(
                "Interactive Items",
                v_flex()
                    .gap_3()
                    .child(
                        Breadcrumb::new()
                            .child("Home")
                            .child(BreadcrumbItem::new("Documents").on_click(cx.listener(
                                |this, _, _, cx| {
                                    this.clicked_item = Some("Documents".to_string());
                                    cx.notify();
                                },
                            )))
                            .child(BreadcrumbItem::new("Projects").on_click(cx.listener(
                                |this, _, _, cx| {
                                    this.clicked_item = Some("Projects".to_string());
                                    cx.notify();
                                },
                            )))
                            .child(BreadcrumbItem::new("Current")),
                    )
                    .when_some(self.clicked_item.as_ref(), |this, item| {
                        this.child(format!("Selected: {item}"))
                    }),
            ))
            .child(StorySection::new(
                "Disabled Item",
                Breadcrumb::new()
                    .child("Home")
                    .child(BreadcrumbItem::new("Unavailable").disabled(true))
                    .child("Current"),
            ))
    }
}
