use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    combobox::{Combobox, ComboboxState},
    scroll::ScrollableElement as _,
    searchable_list::SearchableVec,
    v_flex, Sizable as _,
};

pub struct ComboboxStory {
    single: gpui::Entity<ComboboxState<SearchableVec<&'static str>>>,
    multiple: gpui::Entity<ComboboxState<SearchableVec<&'static str>>>,
}

impl ComboboxStory {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            single: cx.new(|cx| {
                ComboboxState::new(
                    SearchableVec::new(vec!["Apple", "Banana", "Orange", "Pineapple"]),
                    vec![],
                    window,
                    cx,
                )
                .searchable(true)
            }),
            multiple: cx.new(|cx| {
                ComboboxState::new(
                    SearchableVec::new(vec!["React", "Vue", "Svelte", "SolidJS"]),
                    vec![],
                    window,
                    cx,
                )
                .searchable(true)
                .multiple(true)
            }),
        }
    }
}

impl Render for ComboboxStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Combobox"))
                    .child("Autocomplete controls for single and multiple selection."),
            )
            .child(StorySection::new(
                "Single Selection",
                Combobox::new(&self.single)
                    .placeholder("Choose a fruit")
                    .w(px(280.)),
            ))
            .child(StorySection::new(
                "Multiple Selection",
                Combobox::new(&self.multiple)
                    .placeholder("Choose frameworks")
                    .small()
                    .w(px(280.)),
            ))
    }
}
