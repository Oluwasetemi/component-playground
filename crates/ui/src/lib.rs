mod button;
mod button_story;
mod hello_world;
mod story;
mod theme;

use button_story::ButtonStory;
pub use story::{StoryGroup, StoryId};

pub use button::{Button, ButtonVariant, BUTTON_VARIANTS};
pub use hello_world::HelloWorld;
pub use theme::Theme;

use gpui::{div, prelude::*, px, Context, IntoElement, Window};
use gpui_component::{
    h_flex,
    sidebar::{
        Sidebar, SidebarCollapsible, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
        SidebarMenuItem, SidebarToggleButton,
    },
    v_flex, Icon, IconName,
};

pub struct RootView {
    theme: Theme,
    selected: StoryId,
    sidebar_collapsed: bool,
    button_story: gpui::Entity<ButtonStory>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            theme: Theme::groknight(),
            selected: StoryId::Button,
            sidebar_collapsed: false,
            button_story: cx.new(|cx| ButtonStory::new(window, cx)),
        }
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let menu_item = |story: StoryId| {
            SidebarMenuItem::new(story.label())
                .icon(story.icon())
                .active(self.selected == story)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.selected = story;
                    cx.notify();
                }))
        };

        let mut sidebar = Sidebar::new("playground-sidebar")
            .w(px(232.))
            .collapsible(SidebarCollapsible::Icon)
            .collapsed(self.sidebar_collapsed)
            .header(
                SidebarHeader::new().child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::GalleryVerticalEnd))
                        .child(
                            v_flex()
                                .gap_0()
                                .child("Component Playground")
                                .child("Longbridge"),
                        ),
                ),
            );

        for group in StoryGroup::ALL {
            let items = StoryId::ALL
                .iter()
                .copied()
                .filter(|story| story.group() == group)
                .map(&menu_item);
            sidebar = sidebar
                .child(SidebarGroup::new(group.label()).child(SidebarMenu::new().children(items)));
        }

        sidebar.footer(
            SidebarFooter::new().child(
                h_flex()
                    .gap_2()
                    .child(IconName::Info)
                    .child("Component gallery"),
            ),
        )
    }

    fn render_content(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.selected == StoryId::Button {
            self.button_story.clone().into_any_element()
        } else {
            let description = self
                .selected
                .description()
                .unwrap_or("This playground section is ready for its story.");
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .gap_2()
                .child(self.selected.label())
                .child(description)
                .into_any_element()
        }
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(self.theme.window_fill())
            .text_color(self.theme.foreground)
            .child(
                div()
                    .flex()
                    .size_full()
                    .overflow_hidden()
                    .child(self.render_sidebar(cx))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .min_w_0()
                            .bg(self.theme.main_bg)
                            .child(
                                h_flex()
                                    .h(px(52.))
                                    .flex_shrink_0()
                                    .items_center()
                                    .gap_2()
                                    .px(px(16.))
                                    .border_b_1()
                                    .border_color(self.theme.border)
                                    .child(
                                        SidebarToggleButton::new()
                                            .collapsed(self.sidebar_collapsed)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.sidebar_collapsed = !this.sidebar_collapsed;
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        v_flex()
                                            .gap_0()
                                            .child("Component Playground")
                                            .text_color(self.theme.black)
                                            .child(
                                                div()
                                                    .text_size(px(12.))
                                                    .text_color(self.theme.black)
                                                    .child(self.selected.label()),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .min_h_0()
                                    .overflow_hidden()
                                    .child(self.render_content(cx)),
                            ),
                    ),
            )
    }
}
