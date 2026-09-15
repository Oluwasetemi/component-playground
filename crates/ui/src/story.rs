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

    pub const fn icon(self) -> IconName {
        match self {
            Self::BasicControls => IconName::LayoutDashboard,
            Self::FormsAndInputs => IconName::File,
            Self::OverlaysAndMenus => IconName::Menu,
            Self::DataAndLayout => IconName::PanelLeft,
            Self::ContentAndVisualization => IconName::ChartPie,
            Self::Infrastructure => IconName::Settings2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoryId {
    Accordion,
    Alert,
    Attachment,
    AvatarAndAvatarGroup,
    Badge,
    Breadcrumb,
    Bubble,
    Button,
    Carousel,
    Checkbox,
    Collapsible,
    DropdownButton,
    Empty,
    FocusTrap,
    GroupBox,
    Image,
    Kbd,
    Label,
    Link,
    Marker,
    Message,
    MessageScroller,
    Progress,
    Radio,
    Rating,
    Separator,
    Skeleton,
    Shimmer,
    Spinner,
    StatusBar,
    Stepper,
    Switch,
    Tag,
    TabsAndTabBar,
    Toggle,
    Editor,
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
    Command,
    NativeMenu,
    List,
    SearchableList,
    Scrollable,
    Table,
    DataTable,
    DescriptionList,
    Tree,
    VirtualList,
    Scrollbar,
    ResizablePanels,
    Resizable,
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
    pub const ALL: [Self; 94] = [
        Self::Accordion,
        Self::Alert,
        Self::Attachment,
        Self::AvatarAndAvatarGroup,
        Self::Badge,
        Self::Breadcrumb,
        Self::Bubble,
        Self::Button,
        Self::Carousel,
        Self::Checkbox,
        Self::Collapsible,
        Self::DropdownButton,
        Self::Empty,
        Self::FocusTrap,
        Self::GroupBox,
        Self::Image,
        Self::Kbd,
        Self::Label,
        Self::Link,
        Self::Marker,
        Self::Message,
        Self::MessageScroller,
        Self::Progress,
        Self::Radio,
        Self::Rating,
        Self::Separator,
        Self::Skeleton,
        Self::Shimmer,
        Self::Spinner,
        Self::StatusBar,
        Self::Stepper,
        Self::Switch,
        Self::Tag,
        Self::TabsAndTabBar,
        Self::Toggle,
        Self::Editor,
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
        Self::Command,
        Self::NativeMenu,
        Self::List,
        Self::SearchableList,
        Self::Scrollable,
        Self::Table,
        Self::DataTable,
        Self::DescriptionList,
        Self::Tree,
        Self::VirtualList,
        Self::Scrollbar,
        Self::ResizablePanels,
        Self::Resizable,
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
            Self::Attachment => "Attachment",
            Self::AvatarAndAvatarGroup => "Avatar and Avatar Group",
            Self::Badge => "Badge",
            Self::Breadcrumb => "Breadcrumb",
            Self::Bubble => "Bubble",
            Self::Button => "Button and Button Group",
            Self::Carousel => "Carousel",
            Self::Checkbox => "Checkbox",
            Self::Collapsible => "Collapsible",
            Self::DropdownButton => "Dropdown Button",
            Self::Empty => "Empty",
            Self::FocusTrap => "Focus Trap",
            Self::GroupBox => "Group Box",
            Self::Image => "Image",
            Self::Kbd => "Kbd",
            Self::Label => "Label",
            Self::Link => "Link",
            Self::Marker => "Marker",
            Self::Message => "Message",
            Self::MessageScroller => "Message Scroller",
            Self::Progress => "Progress",
            Self::Radio => "Radio",
            Self::Rating => "Rating",
            Self::Separator => "Separator",
            Self::Skeleton => "Skeleton",
            Self::Shimmer => "Shimmer",
            Self::Spinner => "Spinner",
            Self::StatusBar => "Status Bar",
            Self::Stepper => "Stepper",
            Self::Switch => "Switch",
            Self::Tag => "Tag",
            Self::TabsAndTabBar => "Tabs and Tab Bar",
            Self::Toggle => "Toggle",
            Self::Editor => "Editor",
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
            Self::Command => "Command",
            Self::NativeMenu => "Native Menu",
            Self::List => "List",
            Self::SearchableList => "Searchable List",
            Self::Scrollable => "Scrollable",
            Self::Table => "Table",
            Self::DataTable => "Data Table",
            Self::DescriptionList => "Description List",
            Self::Tree => "Tree",
            Self::VirtualList => "Virtual List",
            Self::Scrollbar => "Scrollbar",
            Self::ResizablePanels => "Resizable Panels",
            Self::Resizable => "Resizable",
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
            | Self::Attachment
            | Self::AvatarAndAvatarGroup
            | Self::Badge
            | Self::Breadcrumb
            | Self::Bubble
            | Self::Button
            | Self::Carousel
            | Self::Checkbox
            | Self::Collapsible
            | Self::DropdownButton
            | Self::Empty
            | Self::FocusTrap
            | Self::GroupBox
            | Self::Image
            | Self::Kbd
            | Self::Label
            | Self::Link
            | Self::Marker
            | Self::Message
            | Self::MessageScroller
            | Self::Progress
            | Self::Radio
            | Self::Rating
            | Self::Separator
            | Self::Skeleton
            | Self::Shimmer
            | Self::Spinner
            | Self::StatusBar
            | Self::Stepper
            | Self::Switch
            | Self::Tag
            | Self::TabsAndTabBar
            | Self::Toggle => StoryGroup::BasicControls,
            Self::Editor
            | Self::Input
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
            | Self::Command
            | Self::NativeMenu => StoryGroup::OverlaysAndMenus,
            Self::List
            | Self::SearchableList
            | Self::Scrollable
            | Self::Table
            | Self::DataTable
            | Self::DescriptionList
            | Self::Tree
            | Self::VirtualList
            | Self::Scrollbar
            | Self::ResizablePanels
            | Self::Sidebar
            | Self::DockLayout
            | Self::Resizable => StoryGroup::DataAndLayout,
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
            Self::Attachment | Self::Empty | Self::Image | Self::DescriptionList => IconName::File,
            Self::AvatarAndAvatarGroup => IconName::CircleUser,
            Self::Badge | Self::Rating | Self::Tag => IconName::Star,
            Self::Breadcrumb => IconName::ChevronRight,
            Self::Bubble | Self::Message => IconName::Info,
            Self::Button => IconName::LayoutDashboard,
            Self::Carousel => IconName::LayoutDashboard,
            Self::Checkbox => IconName::Check,
            Self::DropdownButton | Self::Command => IconName::Menu,
            Self::FocusTrap => IconName::Frame,
            Self::GroupBox | Self::WindowBorder => IconName::Frame,
            Self::Editor => IconName::SquareTerminal,
            Self::Kbd | Self::SyntaxHighlighter => IconName::SquareTerminal,
            Self::Link => IconName::ExternalLink,
            Self::Marker => IconName::Minus,
            Self::MessageScroller => IconName::Menu,
            Self::Progress => IconName::LoaderCircle,
            Self::Radio => IconName::CircleCheck,
            Self::Separator => IconName::Minus,
            Self::Skeleton | Self::Spinner | Self::Shimmer | Self::AnimationAndTransitions => {
                IconName::Loader
            }
            Self::StatusBar | Self::PieChart => IconName::ChartPie,
            Self::Stepper | Self::Pagination => IconName::ArrowRight,
            Self::Switch | Self::Slider | Self::Settings => IconName::Settings2,
            Self::TabsAndTabBar | Self::Sidebar => IconName::PanelLeft,
            Self::Toggle => IconName::Settings2,
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
            Self::List | Self::Scrollable | Self::VirtualList => IconName::Menu,
            Self::Tree => IconName::FolderOpen,
            Self::Scrollbar => IconName::PanelRight,
            Self::ResizablePanels | Self::Resizable => IconName::ResizeCorner,
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
