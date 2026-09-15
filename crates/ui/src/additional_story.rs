use crate::StorySection;
use gpui::{
    div, img, prelude::*, px, Axis, Context, FocusHandle, IntoElement, ObjectFit, Render, Window,
};
use gpui_component::{
    attachment::{
        Attachment, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
        AttachmentStatus, AttachmentTitle,
    },
    bubble::{
        Bubble, BubbleContent, BubbleGroup, BubbleReactionSide, BubbleReactions, BubbleVariant,
    },
    button::{
        Button, ButtonVariants as _, DropdownButton, Toggle, ToggleGroup, ToggleVariants as _,
    },
    carousel::{
        Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPagination,
        CarouselPaginationItem, CarouselPrevious, CarouselState,
    },
    command::{Command, CommandGroup, CommandItem, CommandState},
    description_list::{DescriptionItem, DescriptionList},
    empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    },
    h_flex,
    input::{Editor, EditorState},
    marker::{Marker, MarkerContent, MarkerIcon, MarkerLoadingStyle, MarkerVariant},
    message::{
        Message, MessageAlignment, MessageContent, MessageFooter, MessageGroup, MessageHeader,
    },
    message_scroller::{MessageScroller, MessageScrollerState},
    resizable::{h_resizable, resizable_panel},
    scroll::ScrollableElement as _,
    shimmer::{ShimmerStyle, ShimmerText},
    v_flex, ActiveTheme as _, Disableable as _, FocusTrapElement as _, Icon, IconName,
    Sizable as _,
};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdditionalStoryKind {
    Attachment,
    Bubble,
    Carousel,
    DropdownButton,
    Empty,
    FocusTrap,
    Image,
    Marker,
    Message,
    MessageScroller,
    Shimmer,
    Toggle,
    Editor,
    Command,
    Scrollable,
    DescriptionList,
    Resizable,
}

pub struct AdditionalStory {
    kind: AdditionalStoryKind,
    carousel: gpui::Entity<CarouselState>,
    command: gpui::Entity<CommandState>,
    editor: gpui::Entity<EditorState>,
    message_scroller: gpui::Entity<MessageScrollerState>,
    focus_handle: FocusHandle,
    primary_toggle: bool,
    group_toggles: Vec<bool>,
}

impl AdditionalStory {
    pub fn new(kind: AdditionalStoryKind, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            kind,
            carousel: cx.new(|_| CarouselState::new(4).with_looping(true)),
            command: cx.new(|cx| CommandState::new(window, cx)),
            editor: cx.new(|cx| {
                EditorState::new(window, cx)
                    .language("rust")
                    .line_number(true)
                    .folding(true)
                    .default_value("fn main() {\n    println!(\"Hello, GPUI Kit!\");\n}\n")
            }),
            message_scroller: cx.new(|cx| MessageScrollerState::new(8, cx)),
            focus_handle: cx.focus_handle(),
            primary_toggle: false,
            group_toggles: vec![true, false, false],
        }
    }

    fn shell(
        title: &'static str,
        description: &'static str,
        content: impl IntoElement,
    ) -> gpui::AnyElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child(title))
                    .child(description),
            )
            .child(content)
            .into_any_element()
    }

    fn panel(content: impl IntoElement) -> impl IntoElement {
        v_flex()
            .w_full()
            .max_w(px(720.))
            .gap_3()
            .p_4()
            .border_1()
            .border_color(gpui::rgba(0x303030ff))
            .rounded(px(10.))
            .child(content)
    }

    fn render_attachment(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        let complete = Attachment::new()
            .media(AttachmentMedia::new().child(Icon::new(IconName::FileText)))
            .content(
                AttachmentContent::new()
                    .title(AttachmentTitle::new("quarterly-report.pdf"))
                    .description(AttachmentDescription::new("PDF - 2.4 MB")),
            )
            .actions(
                AttachmentActions::new().child(
                    Button::new("remove-report")
                        .ghost()
                        .xsmall()
                        .icon(IconName::Close)
                        .label("Remove"),
                ),
            );
        let uploading = Attachment::new()
            .status(AttachmentStatus::Uploading)
            .media(
                AttachmentMedia::new()
                    .child(Icon::new(IconName::FileText))
                    .overlay(Icon::new(IconName::Loader)),
            )
            .content(
                AttachmentContent::new()
                    .title(AttachmentTitle::new("design-assets.zip"))
                    .description(AttachmentDescription::new("Uploading - 68%"))
                    .child(gpui_component::progress::Progress::new("upload-progress").value(68.)),
            )
            .actions(
                AttachmentActions::new().child(
                    Button::new("cancel-upload")
                        .ghost()
                        .xsmall()
                        .label("Cancel"),
                ),
            );
        let failed = Attachment::new()
            .status(AttachmentStatus::Failed)
            .content(
                AttachmentContent::new()
                    .title(AttachmentTitle::new("archive.zip"))
                    .description(AttachmentDescription::new("Upload failed")),
            )
            .actions(
                AttachmentActions::new()
                    .child(
                        Button::new("retry-upload")
                            .outline()
                            .xsmall()
                            .label("Retry"),
                    )
                    .child(
                        Button::new("remove-failed")
                            .ghost()
                            .xsmall()
                            .label("Remove"),
                    ),
            );

        Self::shell(
            "Attachment",
            "Composed file metadata, media previews, lifecycle states, and actions.",
            v_flex()
                .gap_6()
                .child(StorySection::new("Complete", Self::panel(complete)))
                .child(StorySection::new("Uploading", Self::panel(uploading)))
                .child(StorySection::new("Failed", Self::panel(failed)))
                .child(StorySection::new(
                    "Vertical media",
                    Self::panel(
                        Attachment::new()
                            .axis(Axis::Vertical)
                            .large()
                            .media(AttachmentMedia::new().src(
                                "https://images.unsplash.com/photo-1517694712202-14dd9538aa97?w=640",
                            ))
                            .content(
                                AttachmentContent::new()
                                    .title(AttachmentTitle::new("workspace-preview.png"))
                                        .description(AttachmentDescription::new("PNG - 1920 x 1080")),
                            ),
                    ),
                )),
        )
    }

    fn render_bubble(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        let reactions = BubbleReactions::new()
            .side(BubbleReactionSide::Bottom)
            .action(
                Button::new("bubble-like")
                    .ghost()
                    .xsmall()
                    .label("Like · 2"),
            )
            .action(Button::new("bubble-copy").ghost().xsmall().label("Copy"));
        Self::shell(
            "Bubble",
            "Conversation surfaces with variants, alignment, grouping, and reactions.",
            v_flex()
                .gap_6()
                .child(StorySection::new(
                    "Variants",
                    Self::panel(
                        v_flex()
                            .gap_2()
                            .child(Bubble::new().child("Filled response"))
                            .child(
                                Bubble::new()
                                    .with_variant(BubbleVariant::Secondary)
                                    .child("Secondary incoming message"),
                            )
                            .child(
                                Bubble::new()
                                    .with_variant(BubbleVariant::Outline)
                                    .child("Outlined response"),
                            )
                            .child(
                                Bubble::new()
                                    .with_variant(BubbleVariant::Ghost)
                                    .child("Ghost response spans the available row"),
                            )
                            .child(
                                Bubble::new()
                                    .with_variant(BubbleVariant::Destructive)
                                    .child("Destructive response with an actionable error"),
                            ),
                    ),
                ))
                .child(StorySection::new(
                    "Grouped conversation",
                    Self::panel(
                        BubbleGroup::new()
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Secondary)
                                    .child("The first message belongs to Alice."),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::Start)
                                    .with_variant(BubbleVariant::Secondary)
                                    .child("The follow-up uses the same group."),
                            )
                            .child(
                                Bubble::new()
                                    .alignment(MessageAlignment::End)
                                    .reactions(reactions)
                                    .content(BubbleContent::new().child("This one has reactions.")),
                            ),
                    ),
                )),
        )
    }

    fn render_carousel(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        let state = self.carousel.clone();
        let content_state = state.clone();
        let pagination_state = state.clone();
        Self::shell(
            "Carousel",
            "Looping, keyboard-navigable slides with previous, next, and pagination controls.",
            v_flex().gap_6().child(StorySection::new(
                "Looping project carousel",
                Self::panel(
                    Carousel::new("project-carousel", &state)
                        .accessibility_label("Projects")
                        .child(CarouselContent::new(&content_state).h(px(180.)).children(
                            (0..4).map(|index| {
                                CarouselItem::new(("project-slide", index), index, &content_state)
                                    .child(
                                        v_flex()
                                            .size_full()
                                            .items_center()
                                            .justify_center()
                                            .gap_2()
                                            .bg(gpui::rgba(0x252525ff))
                                            .rounded(px(8.))
                                            .child(Icon::new(IconName::FolderOpen).size_8())
                                            .child(format!("Project {}", index + 1)),
                                    )
                            }),
                        ))
                        .child(CarouselPrevious::new(&state))
                        .child(CarouselNext::new(&state))
                        .child(CarouselPagination::new().children((0..4).map(|index| {
                            CarouselPaginationItem::new(
                                ("project-page", index),
                                index,
                                &pagination_state,
                            )
                            .child((index + 1).to_string())
                        }))),
                ),
            )),
        )
    }

    fn render_dropdown_button(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Dropdown Button",
            "A primary action paired with an independently interactive menu trigger.",
            StorySection::new(
                "Actions",
                Self::panel(
                    h_flex()
                        .gap_3()
                        .child(
                            DropdownButton::new("save-dropdown")
                                .primary()
                                .button(Button::new("save-action").label("Save"))
                                .dropdown_menu(|menu, _, _| {
                                    menu.item(gpui_component::menu::PopupMenuItem::new(
                                        "Save as...",
                                    ))
                                    .item(gpui_component::menu::PopupMenuItem::new("Duplicate"))
                                    .separator()
                                    .item(gpui_component::menu::PopupMenuItem::new("Export"))
                                }),
                        )
                        .child(
                            DropdownButton::new("more-dropdown")
                                .outline()
                                .button(Button::new("more-action").label("More"))
                                .dropdown_menu(|menu, _, _| {
                                    menu.item(gpui_component::menu::PopupMenuItem::new("Share"))
                                        .item(gpui_component::menu::PopupMenuItem::new("Move"))
                                        .item(gpui_component::menu::PopupMenuItem::new("Archive"))
                                }),
                        ),
                ),
            ),
        )
    }

    fn render_empty(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Empty",
            "Structured first-use and no-results states with media, copy, and actions.",
            StorySection::new(
                "No projects",
                Self::panel(
                    Empty::new()
                        .header(
                            EmptyHeader::new()
                                .media(
                                    EmptyMedia::new()
                                        .with_variant(EmptyMediaVariant::Icon)
                                        .child(Icon::new(IconName::Folder)),
                                )
                                .title(EmptyTitle::new().child("No projects yet"))
                                .description(
                                    EmptyDescription::new()
                                        .child("Create your first project to get started."),
                                ),
                        )
                        .content(
                            EmptyContent::new()
                                .flex_row()
                                .gap_2()
                                .child(
                                    Button::new("create-project")
                                        .primary()
                                        .label("Create project"),
                                )
                                .child(Button::new("import-project").outline().label("Import")),
                        )
                        .child("Need help? Read the getting started guide."),
                ),
            ),
        )
    }

    fn render_focus_trap(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Focus Trap",
            "A custom modal-like region that keeps keyboard focus within its controls.",
            StorySection::new(
                "Keyboard boundary",
                Self::panel(
                    v_flex()
                        .gap_3()
                        .p_4()
                        .bg(gpui::rgba(0x242424ff))
                        .rounded(px(8.))
                        .child("Tab cycles through these actions instead of escaping the panel.")
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Button::new("trap-save").primary().label("Save"))
                                .child(Button::new("trap-cancel").outline().label("Cancel"))
                                .child(Button::new("trap-reset").ghost().label("Reset")),
                        )
                        .focus_trap("playground-focus-trap", &self.focus_handle),
                ),
            ),
        )
    }

    fn render_image(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        let image = |source: &'static str, fit: ObjectFit| {
            div()
                .relative()
                .w(px(210.))
                .h(px(140.))
                .overflow_hidden()
                .rounded(px(8.))
                .border_1()
                .border_color(gpui::rgba(0x383838ff))
                .child(img(source).size_full().object_fit(fit))
        };
        Self::shell(
            "Image",
            "Native image rendering with sizing, object-fit choices, and a visible fallback state.",
            v_flex()
                .gap_6()
                .child(StorySection::new(
                    "Object fit",
                    Self::panel(
                        h_flex()
                            .gap_4()
                            .child(image(
                                "https://images.unsplash.com/photo-1518770660439-4636190af475?w=640",
                                ObjectFit::Cover,
                            ))
                            .child(image(
                                "https://images.unsplash.com/photo-1518770660439-4636190af475?w=640",
                                ObjectFit::Contain,
                            )),
                    ),
                ))
                .child(StorySection::new(
                    "Fallback and loading states",
                    Self::panel(
                        h_flex()
                            .gap_4()
                            .child(
                                div()
                                    .size(px(140.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(gpui::rgba(0x282828ff))
                                    .rounded(px(8.))
                                    .child(Icon::new(IconName::File).size_8()),
                            )
                            .child(
                                div()
                                    .size(px(140.))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(gpui::rgba(0x282828ff))
                                    .rounded(px(8.))
                                    .child("Loading…"),
                            ),
                    ),
                )),
        )
    }

    fn render_marker(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Marker",
            "Compact status rows with icons, separators, borders, and loading treatments.",
            StorySection::new(
                "Variants and loading",
                Self::panel(
                    v_flex()
                        .gap_3()
                        .child(
                            Marker::new()
                                .icon(MarkerIcon::new().child(Icon::new(IconName::CircleCheck)))
                                .content(MarkerContent::new().text("Synced")),
                        )
                        .child(
                            Marker::new()
                                .with_variant(MarkerVariant::Separator)
                                .content(MarkerContent::new().text("Today")),
                        )
                        .child(
                            Marker::new()
                                .with_variant(MarkerVariant::Border)
                                .icon(MarkerIcon::new().child(Icon::new(IconName::Info)))
                                .content(MarkerContent::new().text("3 unread messages")),
                        )
                        .child(
                            Marker::new()
                                .loading(true)
                                .with_loading_style(MarkerLoadingStyle::Spinner)
                                .content(MarkerContent::new().text("Loading messages…")),
                        )
                        .child(
                            Marker::new()
                                .loading(true)
                                .with_loading_style(MarkerLoadingStyle::Shimmer)
                                .content(MarkerContent::new().text("Generating response…")),
                        ),
                ),
            ),
        )
    }

    fn message(sender: &'static str, text: &'static str, alignment: MessageAlignment) -> Message {
        Message::new()
            .alignment(alignment)
            .avatar(Icon::new(IconName::CircleUser).size_8())
            .header(MessageHeader::new().child(sender).child("10:24 AM"))
            .content(
                MessageContent::new().bubble(
                    Bubble::new()
                        .with_variant(if alignment == MessageAlignment::Start {
                            BubbleVariant::Secondary
                        } else {
                            BubbleVariant::Filled
                        })
                        .child(text),
                ),
            )
            .footer(MessageFooter::new().child("Delivered"))
    }

    fn render_message(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Message",
            "Conversation rows combining sender identity, metadata, bubbles, and delivery state.",
            StorySection::new(
                "Message group",
                Self::panel(
                    MessageGroup::new()
                        .child(Self::message(
                            "Alice",
                            "Can you review this draft?",
                            MessageAlignment::Start,
                        ))
                        .child(Self::message(
                            "Alice",
                            "The attachment is included below.",
                            MessageAlignment::Start,
                        ))
                        .child(Self::message(
                            "You",
                            "On it. I will send feedback shortly.",
                            MessageAlignment::End,
                        )),
                ),
            ),
        )
    }

    fn render_message_scroller(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        let state = self.message_scroller.clone();
        Self::shell(
            "Message Scroller",
            "A virtualized conversation viewport with tail following and jump-to-latest behavior.",
            StorySection::new(
                "Conversation viewport",
                Self::panel(
                    MessageScroller::new("conversation-scroller", state, |index, _, _| {
                        div().id(("message-row", index)).min_w_0().pb_4().child(
                            Message::new()
                                .alignment(if index % 2 == 0 {
                                    MessageAlignment::Start
                                } else {
                                    MessageAlignment::End
                                })
                                .content(MessageContent::new().bubble(Bubble::new().child(
                                    format!(
                                        "Virtualized message {} with variable content.",
                                        index + 1
                                    ),
                                ))),
                        )
                    })
                    .h(px(320.))
                    .with_bottom_fade(gpui::rgba(0x171717ff)),
                ),
            ),
        )
    }

    fn render_shimmer(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        let style = ShimmerStyle::new()
            .duration(Duration::from_secs(3))
            .highlight_color(cx.theme().primary)
            .spread(0.45)
            .reverse(true);
        Self::shell(
            "Shimmer",
            "Readable loading text with reusable theme-aware animation configuration.",
            StorySection::new(
                "Loading labels",
                Self::panel(
                    v_flex()
                        .gap_3()
                        .child(ShimmerText::new("Thinking...").text_lg())
                        .child(ShimmerText::new("Indexing files...").with_shimmer_style(style))
                        .child(ShimmerText::new("Finalizing preview...").once(true)),
                ),
            ),
        )
    }

    fn render_toggle(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Toggle",
            "Button-style binary choices with controlled state, variants, and groups.",
            v_flex()
                .gap_6()
                .child(StorySection::new(
                    "Individual toggle",
                    Self::panel(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                Toggle::new("primary-toggle")
                                    .label("Notifications")
                                    .checked(self.primary_toggle)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.primary_toggle = *checked;
                                        cx.notify();
                                    })),
                            )
                            .child("Click to update controlled state"),
                    ),
                ))
                .child(StorySection::new(
                    "Segmented group",
                    Self::panel(
                        ToggleGroup::new("view-filters")
                            .segmented()
                            .outline()
                            .children(self.group_toggles.iter().enumerate().map(
                                |(index, checked)| {
                                    Toggle::new(index)
                                        .label(["All", "Unread", "Pinned"][index])
                                        .checked(*checked)
                                },
                            ))
                            .on_click(cx.listener(|this, states: &Vec<bool>, _, cx| {
                                this.group_toggles = states.clone();
                                cx.notify();
                            })),
                    ),
                )),
        )
    }

    fn render_editor(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Editor",
            "A source editor with language highlighting, line numbers, folding, and search support.",
            StorySection::new(
                "Rust source",
                Self::panel(
                    Editor::new(&self.editor)
                        .h(px(340.))
                        .aria_label("Rust source editor"),
                ),
            ),
        )
    }

    fn render_command(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Command",
            "Filterable command palette with groups, disabled items, and keyboard navigation.",
            StorySection::new(
                "Command palette",
                Self::panel(
                    Command::new(&self.command)
                        .placeholder("Search commands...")
                        .group(
                            CommandGroup::new()
                                .label("Suggestions")
                                .item(
                                    CommandItem::new()
                                        .label("Open project")
                                        .icon(IconName::FolderOpen),
                                )
                                .item(
                                    CommandItem::new()
                                        .label("Search files")
                                        .icon(IconName::Search),
                                )
                                .item(
                                    CommandItem::new()
                                        .label("Settings")
                                        .icon(IconName::Settings2),
                                ),
                        )
                        .separator()
                        .group(
                            CommandGroup::new()
                                .label("Account")
                                .item(CommandItem::new().label("Profile"))
                                .item(CommandItem::new().label("Billing").disabled(true)),
                        )
                        .empty(|_, _, _| div().p_4().child("No matching commands."))
                        .w(px(420.)),
                ),
            ),
        )
    }

    fn render_scrollable(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Scrollable",
            "Explicit vertical and horizontal scroll regions with themed scrollbars.",
            v_flex()
                .gap_6()
                .child(StorySection::new(
                    "Vertical scrolling",
                    Self::panel(
                        v_flex()
                            .h(px(190.))
                            .overflow_y_scrollbar()
                            .gap_2()
                            .children((0..18).map(|index| {
                                div()
                                    .h(px(30.))
                                    .flex_shrink_0()
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .bg(if index % 2 == 0 {
                                        gpui::rgba(0x282828ff)
                                    } else {
                                        gpui::rgba(0x202020ff)
                                    })
                                    .child(format!("Scrollable row {}", index + 1))
                            })),
                    ),
                ))
                .child(StorySection::new(
                    "Horizontal scrolling",
                    Self::panel(
                        h_flex()
                            .h(px(100.))
                            .overflow_x_scrollbar()
                            .gap_3()
                            .children((0..12).map(|index| {
                                div()
                                    .w(px(120.))
                                    .h(px(64.))
                                    .flex_shrink_0()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(gpui::rgba(0x272d3aff))
                                    .rounded(px(6.))
                                    .child(format!("Card {}", index + 1))
                            })),
                    ),
                )),
        )
    }

    fn render_description_list(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Description List",
            "Structured key-value metadata with horizontal, vertical, bordered, and spanning layouts.",
            v_flex()
                .gap_6()
                .child(StorySection::new(
                    "Horizontal metadata",
                    Self::panel(
                        DescriptionList::horizontal()
                            .columns(2)
                            .item("Project", "Component Playground", 1)
                            .item("Version", "0.1.0", 1)
                            .item("Status", "Active", 1)
                            .item("Platform", "macOS", 1)
                            .separator()
                            .child(
                                DescriptionItem::new("Description")
                                    .value("A native GPUI component gallery")
                                    .span(2),
                            ),
                    ),
                ))
                .child(StorySection::new(
                    "Vertical, compact metadata",
                    Self::panel(
                        DescriptionList::vertical()
                            .small()
                            .bordered(false)
                            .item("Operating System", "macOS 14", 1)
                            .item("Architecture", "Apple Silicon", 1)
                            .item("Theme", "Groknight", 1),
                    ),
                )),
        )
    }

    fn render_resizable(&self, _cx: &mut Context<Self>) -> gpui::AnyElement {
        Self::shell(
            "Resizable",
            "Horizontal panel groups with initial sizes, constraints, and draggable handles.",
            StorySection::new(
                "Workspace layout",
                Self::panel(
                    div().h(px(240.)).child(
                        h_resizable("additional-resizable")
                            .child(
                                resizable_panel()
                                    .size(px(160.))
                                    .size_range(px(120.)..px(260.))
                                    .child(v_flex().p_3().gap_2().child("Explorer").child("Files")),
                            )
                            .child(
                                resizable_panel()
                                    .child(v_flex().p_3().gap_2().child("Editor").child("main.rs")),
                            )
                            .child(
                                resizable_panel()
                                    .size(px(180.))
                                    .size_range(px(140.)..px(260.))
                                    .child(
                                        v_flex()
                                            .p_3()
                                            .gap_2()
                                            .child("Inspector")
                                            .child("Properties"),
                                    ),
                            ),
                    ),
                ),
            ),
        )
    }
}

impl Render for AdditionalStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.kind {
            AdditionalStoryKind::Attachment => self.render_attachment(cx),
            AdditionalStoryKind::Bubble => self.render_bubble(cx),
            AdditionalStoryKind::Carousel => self.render_carousel(cx),
            AdditionalStoryKind::DropdownButton => self.render_dropdown_button(cx),
            AdditionalStoryKind::Empty => self.render_empty(cx),
            AdditionalStoryKind::FocusTrap => self.render_focus_trap(cx),
            AdditionalStoryKind::Image => self.render_image(cx),
            AdditionalStoryKind::Marker => self.render_marker(cx),
            AdditionalStoryKind::Message => self.render_message(cx),
            AdditionalStoryKind::MessageScroller => self.render_message_scroller(cx),
            AdditionalStoryKind::Shimmer => self.render_shimmer(cx),
            AdditionalStoryKind::Toggle => self.render_toggle(cx),
            AdditionalStoryKind::Editor => self.render_editor(cx),
            AdditionalStoryKind::Command => self.render_command(cx),
            AdditionalStoryKind::Scrollable => self.render_scrollable(cx),
            AdditionalStoryKind::DescriptionList => self.render_description_list(cx),
            AdditionalStoryKind::Resizable => self.render_resizable(cx),
        }
    }
}
