use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    scroll::ScrollableElement as _,
    searchable_list::SearchableVec,
    select::{Select, SelectState},
    v_flex, IndexPath, Sizable as _,
};

pub struct SelectStory {
    framework: gpui::Entity<SelectState<SearchableVec<&'static str>>>,
    searchable: gpui::Entity<SelectState<SearchableVec<&'static str>>>,
}

impl SelectStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            framework: cx.new(|cx| {
                SelectState::new(
                    SearchableVec::new(vec!["GPUI", "Iced", "egui", "Makepad"]),
                    Some(IndexPath::new(0)),
                    window,
                    cx,
                )
            }),
            searchable: cx.new(|cx| {
                SelectState::new(
                    SearchableVec::new(vec!["Rust", "Go", "C++", "JavaScript", "Swift"]),
                    None,
                    window,
                    cx,
                )
                .searchable(true)
            }),
        }
    }
}

impl Render for SelectStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Select"))
                    .child("Choose one option from a menu, with optional search."),
            )
            .child(StorySection::new(
                "Default Select",
                Select::new(&self.framework).w(px(280.)),
            ))
            .child(StorySection::new(
                "Searchable Select",
                Select::new(&self.searchable)
                    .search_placeholder("Search languages")
                    .small()
                    .w(px(280.)),
            ))
    }
}
