mod accordion_story;
mod alert_story;
mod avatar_story;
mod badge_story;
mod breadcrumb_story;
mod button;
mod button_story;
mod checkbox_story;
mod collapsible_story;
mod group_box_story;
mod hello_world;
mod kbd_story;
mod label_story;
mod link_story;
mod progress_story;
mod radio_story;
mod rating_story;
mod separator_story;
mod skeleton_story;
mod spinner_story;
mod status_bar_story;
mod stepper_story;
mod story;
mod story_section;
mod switch_story;
mod tabs_story;
mod tag_story;
mod theme;

use accordion_story::AccordionStory;
use alert_story::AlertStory;
use avatar_story::AvatarStory;
use badge_story::BadgeStory;
use breadcrumb_story::BreadcrumbStory;
use button_story::ButtonStory;
use checkbox_story::CheckboxStory;
use collapsible_story::CollapsibleStory;
use group_box_story::GroupBoxStory;
use kbd_story::KbdStory;
use label_story::LabelStory;
use link_story::LinkStory;
use progress_story::ProgressStory;
use radio_story::RadioStory;
use rating_story::RatingStory;
use separator_story::SeparatorStory;
use skeleton_story::SkeletonStory;
use spinner_story::SpinnerStory;
use status_bar_story::StatusBarStory;
use stepper_story::StepperStory;
pub use story::{StoryGroup, StoryId};
pub use story_section::StorySection;
use switch_story::SwitchStory;
use tabs_story::TabsStory;
use tag_story::TagStory;

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
    accordion_story: gpui::Entity<AccordionStory>,
    alert_story: gpui::Entity<AlertStory>,
    avatar_story: gpui::Entity<AvatarStory>,
    badge_story: gpui::Entity<BadgeStory>,
    button_story: gpui::Entity<ButtonStory>,
    breadcrumb_story: gpui::Entity<BreadcrumbStory>,
    checkbox_story: gpui::Entity<CheckboxStory>,
    collapsible_story: gpui::Entity<CollapsibleStory>,
    group_box_story: gpui::Entity<GroupBoxStory>,
    kbd_story: gpui::Entity<KbdStory>,
    label_story: gpui::Entity<LabelStory>,
    link_story: gpui::Entity<LinkStory>,
    progress_story: gpui::Entity<ProgressStory>,
    radio_story: gpui::Entity<RadioStory>,
    rating_story: gpui::Entity<RatingStory>,
    separator_story: gpui::Entity<SeparatorStory>,
    skeleton_story: gpui::Entity<SkeletonStory>,
    spinner_story: gpui::Entity<SpinnerStory>,
    status_bar_story: gpui::Entity<StatusBarStory>,
    stepper_story: gpui::Entity<StepperStory>,
    switch_story: gpui::Entity<SwitchStory>,
    tag_story: gpui::Entity<TagStory>,
    tabs_story: gpui::Entity<TabsStory>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            theme: Theme::groknight(),
            selected: StoryId::Button,
            sidebar_collapsed: false,
            accordion_story: cx.new(|cx| AccordionStory::new(window, cx)),
            alert_story: cx.new(|cx| AlertStory::new(window, cx)),
            avatar_story: cx.new(|cx| AvatarStory::new(window, cx)),
            badge_story: cx.new(|cx| BadgeStory::new(window, cx)),
            button_story: cx.new(|cx| ButtonStory::new(window, cx)),
            breadcrumb_story: cx.new(|cx| BreadcrumbStory::new(window, cx)),
            checkbox_story: cx.new(|cx| CheckboxStory::new(window, cx)),
            collapsible_story: cx.new(|cx| CollapsibleStory::new(window, cx)),
            group_box_story: cx.new(|cx| GroupBoxStory::new(window, cx)),
            kbd_story: cx.new(|cx| KbdStory::new(window, cx)),
            label_story: cx.new(|cx| LabelStory::new(window, cx)),
            link_story: cx.new(|cx| LinkStory::new(window, cx)),
            progress_story: cx.new(|cx| ProgressStory::new(window, cx)),
            radio_story: cx.new(|cx| RadioStory::new(window, cx)),
            rating_story: cx.new(|cx| RatingStory::new(window, cx)),
            separator_story: cx.new(|cx| SeparatorStory::new(window, cx)),
            skeleton_story: cx.new(|cx| SkeletonStory::new(window, cx)),
            spinner_story: cx.new(|cx| SpinnerStory::new(window, cx)),
            status_bar_story: cx.new(|cx| StatusBarStory::new(window, cx)),
            stepper_story: cx.new(|cx| StepperStory::new(window, cx)),
            switch_story: cx.new(|cx| SwitchStory::new(window, cx)),
            tag_story: cx.new(|cx| TagStory::new(window, cx)),
            tabs_story: cx.new(|cx| TabsStory::new(window, cx)),
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
        match self.selected {
            StoryId::Accordion => self.accordion_story.clone().into_any_element(),
            StoryId::Alert => self.alert_story.clone().into_any_element(),
            StoryId::AvatarAndAvatarGroup => self.avatar_story.clone().into_any_element(),
            StoryId::Badge => self.badge_story.clone().into_any_element(),
            StoryId::Button => self.button_story.clone().into_any_element(),
            StoryId::Breadcrumb => self.breadcrumb_story.clone().into_any_element(),
            StoryId::Checkbox => self.checkbox_story.clone().into_any_element(),
            StoryId::Collapsible => self.collapsible_story.clone().into_any_element(),
            StoryId::GroupBox => self.group_box_story.clone().into_any_element(),
            StoryId::Kbd => self.kbd_story.clone().into_any_element(),
            StoryId::Label => self.label_story.clone().into_any_element(),
            StoryId::Link => self.link_story.clone().into_any_element(),
            StoryId::Progress => self.progress_story.clone().into_any_element(),
            StoryId::Radio => self.radio_story.clone().into_any_element(),
            StoryId::Rating => self.rating_story.clone().into_any_element(),
            StoryId::Separator => self.separator_story.clone().into_any_element(),
            StoryId::Skeleton => self.skeleton_story.clone().into_any_element(),
            StoryId::Spinner => self.spinner_story.clone().into_any_element(),
            StoryId::StatusBar => self.status_bar_story.clone().into_any_element(),
            StoryId::Stepper => self.stepper_story.clone().into_any_element(),
            StoryId::Switch => self.switch_story.clone().into_any_element(),
            StoryId::Tag => self.tag_story.clone().into_any_element(),
            StoryId::TabsAndTabBar => self.tabs_story.clone().into_any_element(),
            story => {
                let description = story
                    .description()
                    .unwrap_or("This playground section is ready for its story.");
                v_flex()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(story.label())
                    .child(description)
                    .into_any_element()
            }
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
                        v_flex()
                            .size_full()
                            .min_w_0()
                            .bg(self.theme.main_bg)
                            .text_color(self.theme.foreground)
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
