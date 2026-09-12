use crate::StorySection;
use gpui::{div, prelude::*, px, App, Context, IntoElement, Render, Window};
use gpui_component::{
    scroll::ScrollableElement as _,
    setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    v_flex, Icon, IconName,
};

pub struct SettingsStory;

impl SettingsStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for SettingsStory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Settings"))
                    .child("Searchable pages and grouped application preferences."),
            )
            .child(StorySection::new(
                "Settings Panel",
                div().w_full().h(px(420.)).child(
                    Settings::new("settings-story").page(
                        SettingPage::new("General")
                            .default_open(true)
                            .icon(Icon::new(IconName::Settings2))
                            .group(
                                SettingGroup::new()
                                    .title("Preferences")
                                    .description("Common application behavior")
                                    .item(
                                        SettingItem::new(
                                            "Notifications",
                                            SettingField::switch(
                                                |_: &App| true,
                                                |_: bool, _: &mut App| {},
                                            ),
                                        )
                                        .description("Show updates and reminders."),
                                    )
                                    .item(
                                        SettingItem::new(
                                            "Compact layout",
                                            SettingField::checkbox(
                                                |_: &App| false,
                                                |_: bool, _: &mut App| {},
                                            ),
                                        )
                                        .description("Use tighter spacing in the interface."),
                                    ),
                            ),
                    ),
                ),
            ))
    }
}
