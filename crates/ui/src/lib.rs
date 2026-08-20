mod button;
mod button_story;
mod hello_world;
mod theme;

use button_story::ButtonStory;

pub use button::{Button, ButtonVariant, BUTTON_VARIANTS};
pub use hello_world::HelloWorld;
pub use theme::Theme;

use gpui::{div, prelude::*, px, Context, IntoElement, SharedString, Window};
use gpui_component::{
    h_flex,
    sidebar::{
        Sidebar, SidebarCollapsible, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu,
        SidebarMenuItem, SidebarToggleButton,
    },
    v_flex, Icon, IconName,
};

fn component_icon(label: &str) -> IconName {
    match label {
        "Accordion" | "Collapsible" => IconName::ChevronDown,
        "Alert" | "Alert Dialog" => IconName::Info,
        "Avatar and Avatar Group" => IconName::CircleUser,
        "Badge" | "Rating" | "Tag" => IconName::Star,
        "Breadcrumb" => IconName::ChevronRight,
        "Button and Button Group" => IconName::LayoutDashboard,
        "Checkbox" => IconName::Check,
        "Group Box" | "Window Border" => IconName::Frame,
        "Kbd" | "Syntax Highlighter" => IconName::SquareTerminal,
        "Label" | "Text View" => IconName::Info,
        "Link" => IconName::ExternalLink,
        "Progress" => IconName::LoaderCircle,
        "Radio" => IconName::CircleCheck,
        "Separator" => IconName::Minus,
        "Skeleton" | "Spinner" | "Animation and Transitions" => IconName::Loader,
        "Status Bar" | "Pie Chart" => IconName::ChartPie,
        "Stepper" | "Pagination" => IconName::ArrowRight,
        "Switch" | "Slider" | "Settings" => IconName::Settings2,
        "Tabs and Tab Bar" | "Sidebar" => IconName::PanelLeft,
        "Input" | "Searchable List" => IconName::Search,
        "Textarea" | "Markdown" | "Table" | "Data Table" => IconName::File,
        "OTP Input" => IconName::Asterisk,
        "Number Input" => IconName::Plus,
        "Select" | "Date Picker" | "Calendar" => IconName::Calendar,
        "Combobox" => IconName::Search,
        "Color Picker" | "Theme" => IconName::Palette,
        "Form" | "Clipboard" => IconName::Copy,
        "Dialog" | "Popover" | "Tooltip" => IconName::Info,
        "Hover Card" => IconName::Eye,
        "Sheet" => IconName::PanelRight,
        "Notification" => IconName::Bell,
        "Menu" | "Dropdown Menu" | "Native Menu" => IconName::Menu,
        "Context Menu" => IconName::Ellipsis,
        "List" | "Virtual List" => IconName::Menu,
        "Tree" => IconName::FolderOpen,
        "Scrollbar" => IconName::PanelRight,
        "Resizable Panels" => IconName::ResizeCorner,
        "Dock Layout" | "Component Root" => IconName::LayoutDashboard,
        "Icon" => IconName::Asterisk,
        "HTML Rendering" => IconName::Globe,
        "Line Chart" | "Bar Chart" | "Area Chart" | "Candlestick Chart" | "Plot" => {
            IconName::ChartPie
        }
        "Radar Chart" | "Sankey Chart" => IconName::Network,
        "Title Bar" => IconName::GalleryVerticalEnd,
        "History" => IconName::Undo2,
        _ => IconName::SquareTerminal,
    }
}

pub struct RootView {
    theme: Theme,
    selected: SharedString,
    sidebar_collapsed: bool,
    button_story: gpui::Entity<ButtonStory>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            theme: Theme::groknight(),
            selected: "Button and Button Group".into(),
            sidebar_collapsed: false,
            button_story: cx.new(|cx| ButtonStory::new(window, cx)),
        }
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let menu_item = |label: &'static str| {
            SidebarMenuItem::new(label)
                .icon(component_icon(label))
                .active(self.selected == label)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.selected = label.into();
                    cx.notify();
                }))
        };

        Sidebar::new("playground-sidebar")
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
            )
            .child(
                SidebarGroup::new("Basic Controls").child(SidebarMenu::new().children([
                    menu_item("Accordion"),
                    menu_item("Alert"),
                    menu_item("Avatar and Avatar Group"),
                    menu_item("Badge"),
                    menu_item("Breadcrumb"),
                    menu_item("Button and Button Group"),
                    menu_item("Checkbox"),
                    menu_item("Collapsible"),
                    menu_item("Group Box"),
                    menu_item("Kbd"),
                    menu_item("Label"),
                    menu_item("Link"),
                    menu_item("Progress"),
                    menu_item("Radio"),
                    menu_item("Rating"),
                    menu_item("Separator"),
                    menu_item("Skeleton"),
                    menu_item("Spinner"),
                    menu_item("Status Bar"),
                    menu_item("Stepper"),
                    menu_item("Switch"),
                    menu_item("Tag"),
                    menu_item("Tabs and Tab Bar"),
                ])),
            )
            .child(
                SidebarGroup::new("Forms and Inputs").child(SidebarMenu::new().children([
                    menu_item("Input"),
                    menu_item("Textarea"),
                    menu_item("OTP Input"),
                    menu_item("Number Input"),
                    menu_item("Select"),
                    menu_item("Combobox"),
                    menu_item("Color Picker"),
                    menu_item("Slider"),
                    menu_item("Form"),
                    menu_item("Settings"),
                    menu_item("Pagination"),
                    menu_item("Date Picker"),
                    menu_item("Calendar"),
                ])),
            )
            .child(
                SidebarGroup::new("Overlays and Menus").child(SidebarMenu::new().children([
                    menu_item("Dialog"),
                    menu_item("Alert Dialog"),
                    menu_item("Popover"),
                    menu_item("Tooltip"),
                    menu_item("Hover Card"),
                    menu_item("Sheet"),
                    menu_item("Notification"),
                    menu_item("Menu"),
                    menu_item("Dropdown Menu"),
                    menu_item("Context Menu"),
                    menu_item("Native Menu"),
                ])),
            )
            .child(
                SidebarGroup::new("Data and Layout").child(SidebarMenu::new().children([
                    menu_item("List"),
                    menu_item("Searchable List"),
                    menu_item("Table"),
                    menu_item("Data Table"),
                    menu_item("Tree"),
                    menu_item("Virtual List"),
                    menu_item("Scrollbar"),
                    menu_item("Resizable Panels"),
                    menu_item("Sidebar"),
                    menu_item("Dock Layout"),
                ])),
            )
            .child(SidebarGroup::new("Content and Visualization").child(
                SidebarMenu::new().children([
                    menu_item("Icon"),
                    menu_item("Text View"),
                    menu_item("Markdown"),
                    menu_item("HTML Rendering"),
                    menu_item("Syntax Highlighter"),
                    menu_item("Line Chart"),
                    menu_item("Bar Chart"),
                    menu_item("Area Chart"),
                    menu_item("Pie Chart"),
                    menu_item("Candlestick Chart"),
                    menu_item("Radar Chart"),
                    menu_item("Sankey Chart"),
                    menu_item("Plot"),
                ]),
            ))
            .child(
                SidebarGroup::new("Infrastructure").child(SidebarMenu::new().children([
                    menu_item("Theme"),
                    menu_item("Animation and Transitions"),
                    menu_item("Window Border"),
                    menu_item("Title Bar"),
                    menu_item("Clipboard"),
                    menu_item("History"),
                    menu_item("Component Root"),
                ])),
            )
            .footer(
                SidebarFooter::new().child(
                    h_flex()
                        .gap_2()
                        .child(IconName::Info)
                        .child("Component gallery"),
                ),
            )
    }

    fn render_content(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.selected == "Button and Button Group" {
            self.button_story.clone().into_any_element()
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .gap_2()
                .child(self.selected.clone())
                .child("This playground section is ready for its story.")
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
                                                    .child(self.selected.clone()),
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
