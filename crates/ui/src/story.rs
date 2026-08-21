use gpui_component::IconName;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoryGroup {
    BasicControls,
    FormsAndInputs,
    OverlaysAndMenus,
    DataAndLayout,
    ContentAndVisualization,
    Infrastructure,
}

impl StoryGroup {
    pub const ALL: [Self; 6] = [
        Self::BasicControls,
        Self::FormsAndInputs,
        Self::OverlaysAndMenus,
        Self::DataAndLayout,
        Self::ContentAndVisualization,
        Self::Infrastructure,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::BasicControls => "Basic Controls",
            Self::FormsAndInputs => "Forms and Inputs",
            Self::OverlaysAndMenus => "Overlays and Menus",
            Self::DataAndLayout => "Data and Layout",
            Self::ContentAndVisualization => "Content and Visualization",
            Self::Infrastructure => "Infrastructure",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoryId {
    Accordion,
    Alert,
    AvatarAndAvatarGroup,
    Badge,
    Breadcrumb,
    Button,
    Checkbox,
    Collapsible,
    GroupBox,
    Kbd,
    Label,
    Link,
    Progress,
    Radio,
    Rating,
    Separator,
    Skeleton,
    Spinner,
    StatusBar,
    Stepper,
    Switch,
    Tag,
    TabsAndTabBar,
    Input,
    Textarea,
    OtpInput,
    NumberInput,
    Select,
    Combobox,
    ColorPicker,
    Slider,
    Form,
    Settings,
    Pagination,
    DatePicker,
    Calendar,
    Dialog,
    AlertDialog,
    Popover,
    Tooltip,
    HoverCard,
    Sheet,
    Notification,
    Menu,
    DropdownMenu,
    ContextMenu,
    NativeMenu,
    List,
    SearchableList,
    Table,
    DataTable,
    Tree,
    VirtualList,
    Scrollbar,
    ResizablePanels,
    Sidebar,
    DockLayout,
    Icon,
    TextView,
    Markdown,
    HtmlRendering,
    SyntaxHighlighter,
    LineChart,
    BarChart,
    AreaChart,
    PieChart,
    CandlestickChart,
    RadarChart,
    SankeyChart,
    Plot,
    Theme,
    AnimationAndTransitions,
    WindowBorder,
    TitleBar,
    Clipboard,
    History,
    ComponentRoot,
}

impl StoryId {
    pub const ALL: [Self; 77] = [
        Self::Accordion,
        Self::Alert,
        Self::AvatarAndAvatarGroup,
        Self::Badge,
        Self::Breadcrumb,
        Self::Button,
        Self::Checkbox,
        Self::Collapsible,
        Self::GroupBox,
        Self::Kbd,
        Self::Label,
        Self::Link,
        Self::Progress,
        Self::Radio,
        Self::Rating,
        Self::Separator,
        Self::Skeleton,
        Self::Spinner,
        Self::StatusBar,
        Self::Stepper,
        Self::Switch,
        Self::Tag,
        Self::TabsAndTabBar,
        Self::Input,
        Self::Textarea,
        Self::OtpInput,
        Self::NumberInput,
        Self::Select,
        Self::Combobox,
        Self::ColorPicker,
        Self::Slider,
        Self::Form,
        Self::Settings,
        Self::Pagination,
        Self::DatePicker,
        Self::Calendar,
        Self::Dialog,
        Self::AlertDialog,
        Self::Popover,
        Self::Tooltip,
        Self::HoverCard,
        Self::Sheet,
        Self::Notification,
        Self::Menu,
        Self::DropdownMenu,
        Self::ContextMenu,
        Self::NativeMenu,
        Self::List,
        Self::SearchableList,
        Self::Table,
        Self::DataTable,
        Self::Tree,
        Self::VirtualList,
        Self::Scrollbar,
        Self::ResizablePanels,
        Self::Sidebar,
        Self::DockLayout,
        Self::Icon,
        Self::TextView,
        Self::Markdown,
        Self::HtmlRendering,
        Self::SyntaxHighlighter,
        Self::LineChart,
        Self::BarChart,
        Self::AreaChart,
        Self::PieChart,
        Self::CandlestickChart,
        Self::RadarChart,
        Self::SankeyChart,
        Self::Plot,
        Self::Theme,
        Self::AnimationAndTransitions,
        Self::WindowBorder,
        Self::TitleBar,
        Self::Clipboard,
        Self::History,
        Self::ComponentRoot,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Accordion => "Accordion",
            Self::Alert => "Alert",
            Self::AvatarAndAvatarGroup => "Avatar and Avatar Group",
            Self::Badge => "Badge",
            Self::Breadcrumb => "Breadcrumb",
            Self::Button => "Button and Button Group",
            Self::Checkbox => "Checkbox",
            Self::Collapsible => "Collapsible",
            Self::GroupBox => "Group Box",
            Self::Kbd => "Kbd",
            Self::Label => "Label",
            Self::Link => "Link",
            Self::Progress => "Progress",
            Self::Radio => "Radio",
            Self::Rating => "Rating",
            Self::Separator => "Separator",
            Self::Skeleton => "Skeleton",
            Self::Spinner => "Spinner",
            Self::StatusBar => "Status Bar",
            Self::Stepper => "Stepper",
            Self::Switch => "Switch",
            Self::Tag => "Tag",
            Self::TabsAndTabBar => "Tabs and Tab Bar",
            Self::Input => "Input",
            Self::Textarea => "Textarea",
            Self::OtpInput => "OTP Input",
            Self::NumberInput => "Number Input",
            Self::Select => "Select",
            Self::Combobox => "Combobox",
            Self::ColorPicker => "Color Picker",
            Self::Slider => "Slider",
            Self::Form => "Form",
            Self::Settings => "Settings",
            Self::Pagination => "Pagination",
            Self::DatePicker => "Date Picker",
            Self::Calendar => "Calendar",
            Self::Dialog => "Dialog",
            Self::AlertDialog => "Alert Dialog",
            Self::Popover => "Popover",
            Self::Tooltip => "Tooltip",
            Self::HoverCard => "Hover Card",
            Self::Sheet => "Sheet",
            Self::Notification => "Notification",
            Self::Menu => "Menu",
            Self::DropdownMenu => "Dropdown Menu",
            Self::ContextMenu => "Context Menu",
            Self::NativeMenu => "Native Menu",
            Self::List => "List",
            Self::SearchableList => "Searchable List",
            Self::Table => "Table",
            Self::DataTable => "Data Table",
            Self::Tree => "Tree",
            Self::VirtualList => "Virtual List",
            Self::Scrollbar => "Scrollbar",
            Self::ResizablePanels => "Resizable Panels",
            Self::Sidebar => "Sidebar",
            Self::DockLayout => "Dock Layout",
            Self::Icon => "Icon",
            Self::TextView => "Text View",
            Self::Markdown => "Markdown",
            Self::HtmlRendering => "HTML Rendering",
            Self::SyntaxHighlighter => "Syntax Highlighter",
            Self::LineChart => "Line Chart",
            Self::BarChart => "Bar Chart",
            Self::AreaChart => "Area Chart",
            Self::PieChart => "Pie Chart",
            Self::CandlestickChart => "Candlestick Chart",
            Self::RadarChart => "Radar Chart",
            Self::SankeyChart => "Sankey Chart",
            Self::Plot => "Plot",
            Self::Theme => "Theme",
            Self::AnimationAndTransitions => "Animation and Transitions",
            Self::WindowBorder => "Window Border",
            Self::TitleBar => "Title Bar",
            Self::Clipboard => "Clipboard",
            Self::History => "History",
            Self::ComponentRoot => "Component Root",
        }
    }

    pub const fn group(self) -> StoryGroup {
        match self {
            Self::Accordion
            | Self::Alert
            | Self::AvatarAndAvatarGroup
            | Self::Badge
            | Self::Breadcrumb
            | Self::Button
            | Self::Checkbox
            | Self::Collapsible
            | Self::GroupBox
            | Self::Kbd
            | Self::Label
            | Self::Link
            | Self::Progress
            | Self::Radio
            | Self::Rating
            | Self::Separator
            | Self::Skeleton
            | Self::Spinner
            | Self::StatusBar
            | Self::Stepper
            | Self::Switch
            | Self::Tag
            | Self::TabsAndTabBar => StoryGroup::BasicControls,
            Self::Input
            | Self::Textarea
            | Self::OtpInput
            | Self::NumberInput
            | Self::Select
            | Self::Combobox
            | Self::ColorPicker
            | Self::Slider
            | Self::Form
            | Self::Settings
            | Self::Pagination
            | Self::DatePicker
            | Self::Calendar => StoryGroup::FormsAndInputs,
            Self::Dialog
            | Self::AlertDialog
            | Self::Popover
            | Self::Tooltip
            | Self::HoverCard
            | Self::Sheet
            | Self::Notification
            | Self::Menu
            | Self::DropdownMenu
            | Self::ContextMenu
            | Self::NativeMenu => StoryGroup::OverlaysAndMenus,
            Self::List
            | Self::SearchableList
            | Self::Table
            | Self::DataTable
            | Self::Tree
            | Self::VirtualList
            | Self::Scrollbar
            | Self::ResizablePanels
            | Self::Sidebar
            | Self::DockLayout => StoryGroup::DataAndLayout,
            Self::Icon
            | Self::TextView
            | Self::Markdown
            | Self::HtmlRendering
            | Self::SyntaxHighlighter
            | Self::LineChart
            | Self::BarChart
            | Self::AreaChart
            | Self::PieChart
            | Self::CandlestickChart
            | Self::RadarChart
            | Self::SankeyChart
            | Self::Plot => StoryGroup::ContentAndVisualization,
            Self::Theme
            | Self::AnimationAndTransitions
            | Self::WindowBorder
            | Self::TitleBar
            | Self::Clipboard
            | Self::History
            | Self::ComponentRoot => StoryGroup::Infrastructure,
        }
    }

    pub const fn icon(self) -> IconName {
        match self {
            Self::Accordion | Self::Collapsible => IconName::ChevronDown,
            Self::Alert | Self::AlertDialog | Self::Label | Self::TextView => IconName::Info,
            Self::AvatarAndAvatarGroup => IconName::CircleUser,
            Self::Badge | Self::Rating | Self::Tag => IconName::Star,
            Self::Breadcrumb => IconName::ChevronRight,
            Self::Button => IconName::LayoutDashboard,
            Self::Checkbox => IconName::Check,
            Self::GroupBox | Self::WindowBorder => IconName::Frame,
            Self::Kbd | Self::SyntaxHighlighter => IconName::SquareTerminal,
            Self::Link => IconName::ExternalLink,
            Self::Progress => IconName::LoaderCircle,
            Self::Radio => IconName::CircleCheck,
            Self::Separator => IconName::Minus,
            Self::Skeleton | Self::Spinner | Self::AnimationAndTransitions => IconName::Loader,
            Self::StatusBar | Self::PieChart => IconName::ChartPie,
            Self::Stepper | Self::Pagination => IconName::ArrowRight,
            Self::Switch | Self::Slider | Self::Settings => IconName::Settings2,
            Self::TabsAndTabBar | Self::Sidebar => IconName::PanelLeft,
            Self::Input | Self::SearchableList | Self::Combobox => IconName::Search,
            Self::Textarea | Self::Markdown | Self::Table | Self::DataTable => IconName::File,
            Self::OtpInput => IconName::Asterisk,
            Self::NumberInput => IconName::Plus,
            Self::Select | Self::DatePicker | Self::Calendar => IconName::Calendar,
            Self::ColorPicker | Self::Theme => IconName::Palette,
            Self::Form | Self::Clipboard => IconName::Copy,
            Self::Dialog | Self::Popover | Self::Tooltip => IconName::Info,
            Self::HoverCard => IconName::Eye,
            Self::Sheet => IconName::PanelRight,
            Self::Notification => IconName::Bell,
            Self::Menu | Self::DropdownMenu | Self::NativeMenu => IconName::Menu,
            Self::ContextMenu => IconName::Ellipsis,
            Self::List | Self::VirtualList => IconName::Menu,
            Self::Tree => IconName::FolderOpen,
            Self::Scrollbar => IconName::PanelRight,
            Self::ResizablePanels => IconName::ResizeCorner,
            Self::DockLayout | Self::ComponentRoot => IconName::LayoutDashboard,
            Self::Icon => IconName::Asterisk,
            Self::HtmlRendering => IconName::Globe,
            Self::LineChart
            | Self::BarChart
            | Self::AreaChart
            | Self::CandlestickChart
            | Self::Plot => IconName::ChartPie,
            Self::RadarChart | Self::SankeyChart => IconName::Network,
            Self::TitleBar => IconName::GalleryVerticalEnd,
            Self::History => IconName::Undo2,
        }
    }

    pub const fn description(self) -> Option<&'static str> {
        match self {
            Self::Accordion => Some("Expandable sections with controlled open state."),
            Self::Alert => Some("Callouts for important status and attention messages."),
            Self::AvatarAndAvatarGroup => Some("User and organization identity representations."),
            Self::Badge => Some("Counts, dots, and status icons attached to content."),
            Self::Button => Some("Displays buttons, button variants, and button groups."),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StoryGroup, StoryId};

    #[test]
    fn registry_has_unique_labels_and_valid_groups() {
        for (index, story) in StoryId::ALL.iter().enumerate() {
            assert!(
                StoryId::ALL[index + 1..]
                    .iter()
                    .all(|other| other.label() != story.label()),
                "duplicate story label: {}",
                story.label()
            );
            assert!(StoryGroup::ALL.contains(&story.group()));
        }
    }
}
