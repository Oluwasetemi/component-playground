mod about;
mod accordion_story;
mod additional_story;
mod alert_dialog_story;
mod alert_story;
mod avatar_story;
mod badge_story;
mod breadcrumb_story;
mod button;
mod button_story;
mod calendar_story;
mod checkbox_story;
mod collapsible_story;
mod color_picker_story;
mod combobox_story;
mod content_visualization_story;
mod context_menu_story;
mod data_layout_story;
mod date_picker_story;
mod dialog_story;
mod dropdown_menu_story;
mod form_story;
mod group_box_story;
mod hello_world;
mod hover_card_story;
mod infrastructure_story;
mod input_story;
mod kbd_story;
mod label_story;
mod link_story;
mod menu_story;
mod native_menu_story;
mod notification_story;
mod number_input_story;
mod otp_input_story;
mod pagination_story;
mod popover_story;
mod progress_story;
mod radio_story;
mod rating_story;
mod select_story;
mod separator_story;
mod settings_story;
mod sheet_story;
mod sidebar_story_group;
mod skeleton_story;
mod slider_story;
mod spinner_story;
mod status_bar_story;
mod stepper_story;
mod story;
mod story_section;
mod switch_story;
mod tabs_story;
mod tag_story;
mod textarea_story;
mod theme;
mod tooltip_story;

use accordion_story::AccordionStory;
use additional_story::{AdditionalStory, AdditionalStoryKind};
use alert_dialog_story::AlertDialogStory;
use alert_story::AlertStory;
use avatar_story::AvatarStory;
use badge_story::BadgeStory;
use breadcrumb_story::BreadcrumbStory;
use button_story::ButtonStory;
use calendar_story::CalendarStory;
use checkbox_story::CheckboxStory;
use collapsible_story::CollapsibleStory;
use color_picker_story::ColorPickerStory;
use combobox_story::ComboboxStory;
use content_visualization_story::{ContentVisualizationKind, ContentVisualizationStory};
use context_menu_story::ContextMenuStory;
use data_layout_story::{DataLayoutKind, DataLayoutStory};
use date_picker_story::DatePickerStory;
use dialog_story::DialogStory;
use dropdown_menu_story::DropdownMenuStory;
use form_story::FormStory;
use gpui_component::input::{Input, InputState};
use group_box_story::GroupBoxStory;
use hover_card_story::HoverCardStory;
use infrastructure_story::{InfrastructureKind, InfrastructureStory};
use input_story::InputStory;
use kbd_story::KbdStory;
use label_story::LabelStory;
use link_story::LinkStory;
use menu_story::MenuStory;
use native_menu_story::NativeMenuStory;
use notification_story::NotificationStory;
use number_input_story::NumberInputStory;
use otp_input_story::OtpInputStory;
use pagination_story::PaginationStory;
use popover_story::PopoverStory;
use progress_story::ProgressStory;
use radio_story::RadioStory;
use rating_story::RatingStory;
use select_story::SelectStory;
use separator_story::SeparatorStory;
use settings_story::SettingsStory;
use sheet_story::SheetStory;
use sidebar_story_group::SidebarStoryGroup;
use skeleton_story::SkeletonStory;
use slider_story::SliderStory;
use spinner_story::SpinnerStory;
use status_bar_story::StatusBarStory;
use stepper_story::StepperStory;
pub use story::{StoryGroup, StoryId};
pub use story_section::StorySection;
use switch_story::SwitchStory;
use tabs_story::TabsStory;
use tag_story::TagStory;
use textarea_story::TextareaStory;
use tooltip_story::TooltipStory;

pub use about::open_about_dialog;
pub use button::{Button, ButtonVariant, BUTTON_VARIANTS};
pub use hello_world::HelloWorld;
pub use theme::Theme;

use gpui::{
    div, img, prelude::*, px, Context, Focusable as _, IntoElement, MouseButton, MouseDownEvent,
    Window,
};
use gpui_component::{
    h_flex,
    sidebar::{
        Sidebar, SidebarCollapsible, SidebarFooter, SidebarHeader, SidebarMenuItem,
        SidebarToggleButton,
    },
    v_flex, Icon, IconName, Root,
};

pub struct RootView {
    theme: Theme,
    selected: StoryId,
    sidebar_collapsed: bool,
    sidebar_open_group: Option<usize>,
    accordion_story: gpui::Entity<AccordionStory>,
    alert_story: gpui::Entity<AlertStory>,
    avatar_story: gpui::Entity<AvatarStory>,
    badge_story: gpui::Entity<BadgeStory>,
    button_story: gpui::Entity<ButtonStory>,
    breadcrumb_story: gpui::Entity<BreadcrumbStory>,
    checkbox_story: gpui::Entity<CheckboxStory>,
    collapsible_story: gpui::Entity<CollapsibleStory>,
    group_box_story: gpui::Entity<GroupBoxStory>,
    sidebar_search: gpui::Entity<InputState>,
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
    input_story: gpui::Entity<InputStory>,
    textarea_story: gpui::Entity<TextareaStory>,
    otp_input_story: gpui::Entity<OtpInputStory>,
    number_input_story: gpui::Entity<NumberInputStory>,
    select_story: gpui::Entity<SelectStory>,
    combobox_story: gpui::Entity<ComboboxStory>,
    color_picker_story: gpui::Entity<ColorPickerStory>,
    slider_story: gpui::Entity<SliderStory>,
    form_story: gpui::Entity<FormStory>,
    settings_story: gpui::Entity<SettingsStory>,
    pagination_story: gpui::Entity<PaginationStory>,
    date_picker_story: gpui::Entity<DatePickerStory>,
    calendar_story: gpui::Entity<CalendarStory>,
    dialog_story: gpui::Entity<DialogStory>,
    alert_dialog_story: gpui::Entity<AlertDialogStory>,
    popover_story: gpui::Entity<PopoverStory>,
    tooltip_story: gpui::Entity<TooltipStory>,
    hover_card_story: gpui::Entity<HoverCardStory>,
    sheet_story: gpui::Entity<SheetStory>,
    notification_story: gpui::Entity<NotificationStory>,
    menu_story: gpui::Entity<MenuStory>,
    dropdown_menu_story: gpui::Entity<DropdownMenuStory>,
    context_menu_story: gpui::Entity<ContextMenuStory>,
    native_menu_story: gpui::Entity<NativeMenuStory>,
    data_layout_stories: Vec<(StoryId, gpui::Entity<DataLayoutStory>)>,
    content_visualization_stories: Vec<(StoryId, gpui::Entity<ContentVisualizationStory>)>,
    infrastructure_stories: Vec<(StoryId, gpui::Entity<InfrastructureStory>)>,
    additional_stories: Vec<(StoryId, gpui::Entity<AdditionalStory>)>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sidebar_search =
            cx.new(|cx| InputState::new(window, cx).placeholder("Search components"));
        cx.observe(&sidebar_search, |_, _, cx| cx.notify()).detach();

        let data_layout_stories = [
            (StoryId::List, DataLayoutKind::List),
            (StoryId::SearchableList, DataLayoutKind::SearchableList),
            (StoryId::Table, DataLayoutKind::Table),
            (StoryId::DataTable, DataLayoutKind::DataTable),
            (StoryId::Tree, DataLayoutKind::Tree),
            (StoryId::VirtualList, DataLayoutKind::VirtualList),
            (StoryId::Scrollbar, DataLayoutKind::Scrollbar),
            (StoryId::ResizablePanels, DataLayoutKind::ResizablePanels),
            (StoryId::Sidebar, DataLayoutKind::Sidebar),
            (StoryId::DockLayout, DataLayoutKind::DockLayout),
        ]
        .into_iter()
        .map(|(story, kind)| (story, cx.new(|cx| DataLayoutStory::new(kind, window, cx))))
        .collect();
        let content_visualization_stories = [
            (StoryId::Icon, ContentVisualizationKind::Icon),
            (StoryId::TextView, ContentVisualizationKind::TextView),
            (StoryId::Markdown, ContentVisualizationKind::Markdown),
            (
                StoryId::HtmlRendering,
                ContentVisualizationKind::HtmlRendering,
            ),
            (
                StoryId::SyntaxHighlighter,
                ContentVisualizationKind::SyntaxHighlighter,
            ),
            (StoryId::LineChart, ContentVisualizationKind::LineChart),
            (StoryId::BarChart, ContentVisualizationKind::BarChart),
            (StoryId::AreaChart, ContentVisualizationKind::AreaChart),
            (StoryId::PieChart, ContentVisualizationKind::PieChart),
            (
                StoryId::CandlestickChart,
                ContentVisualizationKind::CandlestickChart,
            ),
            (StoryId::RadarChart, ContentVisualizationKind::RadarChart),
            (StoryId::SankeyChart, ContentVisualizationKind::SankeyChart),
            (StoryId::Plot, ContentVisualizationKind::Plot),
        ]
        .into_iter()
        .map(|(story, kind)| {
            (
                story,
                cx.new(|cx| ContentVisualizationStory::new(kind, window, cx)),
            )
        })
        .collect();
        let infrastructure_stories = [
            (StoryId::Theme, InfrastructureKind::Theme),
            (
                StoryId::AnimationAndTransitions,
                InfrastructureKind::AnimationAndTransitions,
            ),
            (StoryId::WindowBorder, InfrastructureKind::WindowBorder),
            (StoryId::TitleBar, InfrastructureKind::TitleBar),
            (StoryId::Clipboard, InfrastructureKind::Clipboard),
            (StoryId::History, InfrastructureKind::History),
            (StoryId::ComponentRoot, InfrastructureKind::ComponentRoot),
        ]
        .into_iter()
        .map(|(story, kind)| {
            (
                story,
                cx.new(|cx| InfrastructureStory::new(kind, window, cx)),
            )
        })
        .collect();
        let additional_stories = [
            (StoryId::Attachment, AdditionalStoryKind::Attachment),
            (StoryId::Bubble, AdditionalStoryKind::Bubble),
            (StoryId::Carousel, AdditionalStoryKind::Carousel),
            (StoryId::DropdownButton, AdditionalStoryKind::DropdownButton),
            (StoryId::Empty, AdditionalStoryKind::Empty),
            (StoryId::FocusTrap, AdditionalStoryKind::FocusTrap),
            (StoryId::Image, AdditionalStoryKind::Image),
            (StoryId::Marker, AdditionalStoryKind::Marker),
            (StoryId::Message, AdditionalStoryKind::Message),
            (
                StoryId::MessageScroller,
                AdditionalStoryKind::MessageScroller,
            ),
            (StoryId::Shimmer, AdditionalStoryKind::Shimmer),
            (StoryId::Toggle, AdditionalStoryKind::Toggle),
            (StoryId::Editor, AdditionalStoryKind::Editor),
            (StoryId::Command, AdditionalStoryKind::Command),
            (StoryId::Scrollable, AdditionalStoryKind::Scrollable),
            (
                StoryId::DescriptionList,
                AdditionalStoryKind::DescriptionList,
            ),
            (StoryId::Resizable, AdditionalStoryKind::Resizable),
        ]
        .into_iter()
        .map(|(story, kind)| (story, cx.new(|cx| AdditionalStory::new(kind, window, cx))))
        .collect();

        Self {
            theme: Theme::groknight(),
            selected: StoryId::Button,
            sidebar_collapsed: false,
            sidebar_open_group: Some(0),
            accordion_story: cx.new(|cx| AccordionStory::new(window, cx)),
            alert_story: cx.new(|cx| AlertStory::new(window, cx)),
            avatar_story: cx.new(|cx| AvatarStory::new(window, cx)),
            badge_story: cx.new(|cx| BadgeStory::new(window, cx)),
            button_story: cx.new(|cx| ButtonStory::new(window, cx)),
            breadcrumb_story: cx.new(|cx| BreadcrumbStory::new(window, cx)),
            checkbox_story: cx.new(|cx| CheckboxStory::new(window, cx)),
            collapsible_story: cx.new(|cx| CollapsibleStory::new(window, cx)),
            group_box_story: cx.new(|cx| GroupBoxStory::new(window, cx)),
            sidebar_search,
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
            input_story: cx.new(|cx| InputStory::new(window, cx)),
            textarea_story: cx.new(|cx| TextareaStory::new(window, cx)),
            otp_input_story: cx.new(|cx| OtpInputStory::new(window, cx)),
            number_input_story: cx.new(|cx| NumberInputStory::new(window, cx)),
            select_story: cx.new(|cx| SelectStory::new(window, cx)),
            combobox_story: cx.new(|cx| ComboboxStory::new(window, cx)),
            color_picker_story: cx.new(|cx| ColorPickerStory::new(window, cx)),
            slider_story: cx.new(|cx| SliderStory::new(window, cx)),
            form_story: cx.new(|cx| FormStory::new(window, cx)),
            settings_story: cx.new(|cx| SettingsStory::new(window, cx)),
            pagination_story: cx.new(|cx| PaginationStory::new(window, cx)),
            date_picker_story: cx.new(|cx| DatePickerStory::new(window, cx)),
            calendar_story: cx.new(|cx| CalendarStory::new(window, cx)),
            dialog_story: cx.new(|cx| DialogStory::new(window, cx)),
            alert_dialog_story: cx.new(|cx| AlertDialogStory::new(window, cx)),
            popover_story: cx.new(|cx| PopoverStory::new(window, cx)),
            tooltip_story: cx.new(|cx| TooltipStory::new(window, cx)),
            hover_card_story: cx.new(|cx| HoverCardStory::new(window, cx)),
            sheet_story: cx.new(|cx| SheetStory::new(window, cx)),
            notification_story: cx.new(|cx| NotificationStory::new(window, cx)),
            menu_story: cx.new(|cx| MenuStory::new(window, cx)),
            dropdown_menu_story: cx.new(|cx| DropdownMenuStory::new(window, cx)),
            context_menu_story: cx.new(|cx| ContextMenuStory::new(window, cx)),
            native_menu_story: cx.new(|cx| NativeMenuStory::new(window, cx)),
            data_layout_stories,
            content_visualization_stories,
            infrastructure_stories,
            additional_stories,
        }
    }

    fn render_sidebar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_focused = self
            .sidebar_search
            .read(cx)
            .focus_handle(cx)
            .is_focused(window);
        let search_query = self
            .sidebar_search
            .read(cx)
            .value()
            .trim()
            .to_ascii_lowercase();
        let searching = !search_query.is_empty();
        let menu_item = |story: StoryId| {
            SidebarMenuItem::new(story.label())
                .icon(story.icon())
                .active(self.selected == story)
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
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
                    v_flex()
                        .w_full()
                        .gap_2()
                        .when(self.sidebar_collapsed, |this| {
                            this.child(Icon::new(IconName::GalleryVerticalEnd))
                        })
                        .when(!self.sidebar_collapsed, |this| {
                            this.child(
                                h_flex()
                                    .gap_2()
                                    .child(img("app-icon.png").size(px(24.)).rounded(px(6.)))
                                    .child(
                                        v_flex()
                                            .gap_0()
                                            .child("Component Playground")
                                            .child("Longbridge"),
                                    ),
                            )
                            .child(
                                div()
                                    .rounded(px(8.))
                                    .border_1()
                                    .border_color(if search_focused {
                                        self.theme.accent
                                    } else {
                                        self.theme.border
                                    })
                                    .child(
                                        Input::new(&self.sidebar_search)
                                            .prefix(Icon::new(IconName::Search))
                                            .bordered(false)
                                            .w_full(),
                                    ),
                            )
                        }),
                ),
            );

        for (group_index, group) in StoryGroup::ALL.into_iter().enumerate() {
            let group_matches = group.label().to_ascii_lowercase().contains(&search_query);
            let matching_stories: Vec<_> = StoryId::ALL
                .iter()
                .copied()
                .filter(|story| {
                    story.group() == group
                        && (group_matches
                            || story.label().to_ascii_lowercase().contains(&search_query))
                })
                .collect();
            if searching && matching_stories.is_empty() {
                continue;
            }
            let items = matching_stories.into_iter().map(menu_item);
            sidebar = sidebar.child(
                SidebarStoryGroup::new(group.label(), group.icon(), items)
                    .open(searching || self.sidebar_open_group == Some(group_index))
                    .on_toggle(cx.listener(move |this, _, _, cx| {
                        this.sidebar_open_group = match this.sidebar_open_group {
                            Some(open_group) if open_group == group_index => None,
                            _ => Some(group_index),
                        };
                        cx.notify();
                    })),
            );
        }

        sidebar.footer(
            SidebarFooter::new().child(
                h_flex()
                    .gap_2()
                    .cursor_pointer()
                    .child(IconName::Info)
                    .child("Component gallery")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_, _: &MouseDownEvent, window, cx| {
                            open_about_dialog(window, cx);
                        }),
                    ),
            ),
        )
    }

    fn render_content(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some((_, story)) = self
            .data_layout_stories
            .iter()
            .find(|(id, _)| *id == self.selected)
        {
            return story.clone().into_any_element();
        }
        if let Some((_, story)) = self
            .content_visualization_stories
            .iter()
            .find(|(id, _)| *id == self.selected)
        {
            return story.clone().into_any_element();
        }
        if let Some((_, story)) = self
            .infrastructure_stories
            .iter()
            .find(|(id, _)| *id == self.selected)
        {
            return story.clone().into_any_element();
        }
        if let Some((_, story)) = self
            .additional_stories
            .iter()
            .find(|(id, _)| *id == self.selected)
        {
            return story.clone().into_any_element();
        }

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
            StoryId::Input => self.input_story.clone().into_any_element(),
            StoryId::Textarea => self.textarea_story.clone().into_any_element(),
            StoryId::OtpInput => self.otp_input_story.clone().into_any_element(),
            StoryId::NumberInput => self.number_input_story.clone().into_any_element(),
            StoryId::Select => self.select_story.clone().into_any_element(),
            StoryId::Combobox => self.combobox_story.clone().into_any_element(),
            StoryId::ColorPicker => self.color_picker_story.clone().into_any_element(),
            StoryId::Slider => self.slider_story.clone().into_any_element(),
            StoryId::Form => self.form_story.clone().into_any_element(),
            StoryId::Settings => self.settings_story.clone().into_any_element(),
            StoryId::Pagination => self.pagination_story.clone().into_any_element(),
            StoryId::DatePicker => self.date_picker_story.clone().into_any_element(),
            StoryId::Calendar => self.calendar_story.clone().into_any_element(),
            StoryId::Dialog => self.dialog_story.clone().into_any_element(),
            StoryId::AlertDialog => self.alert_dialog_story.clone().into_any_element(),
            StoryId::Popover => self.popover_story.clone().into_any_element(),
            StoryId::Tooltip => self.tooltip_story.clone().into_any_element(),
            StoryId::HoverCard => self.hover_card_story.clone().into_any_element(),
            StoryId::Sheet => self.sheet_story.clone().into_any_element(),
            StoryId::Notification => self.notification_story.clone().into_any_element(),
            StoryId::Menu => self.menu_story.clone().into_any_element(),
            StoryId::DropdownMenu => self.dropdown_menu_story.clone().into_any_element(),
            StoryId::ContextMenu => self.context_menu_story.clone().into_any_element(),
            StoryId::NativeMenu => self.native_menu_story.clone().into_any_element(),
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx)
            .map(|layer| div().text_color(self.theme.black).child(layer));
        let dialog_layer = Root::render_dialog_layer(window, cx)
            .map(|layer| div().text_color(self.theme.black).child(layer));
        let notification_layer = Root::render_notification_layer(window, cx)
            .map(|layer| div().text_color(self.theme.black).child(layer));

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
                    .child(self.render_sidebar(window, cx))
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
            .children(sheet_layer)
            .children(dialog_layer)
            .children(notification_layer)
    }
}
