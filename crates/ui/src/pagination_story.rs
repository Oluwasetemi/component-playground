use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    pagination::Pagination, scroll::ScrollableElement as _, v_flex, Disableable as _, Sizable as _,
};

pub struct PaginationStory {
    current_page: usize,
}

impl PaginationStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { current_page: 3 }
    }
}

impl Render for PaginationStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Pagination"))
                    .child("Navigate through a finite set of pages."),
            )
            .child(StorySection::new(
                "Interactive Pages",
                Pagination::new("pagination-main")
                    .current_page(self.current_page)
                    .total_pages(8)
                    .with_size(gpui_component::Size::Medium)
                    .on_click(cx.listener(|this, page, _, cx| {
                        this.current_page = *page;
                        cx.notify();
                    })),
            ))
            .child(StorySection::new(
                "Compact and Disabled",
                Pagination::new("pagination-compact")
                    .compact()
                    .current_page(4)
                    .total_pages(10)
                    .small()
                    .disabled(true),
            ))
            .child(format!("Current page: {}", self.current_page))
    }
}
