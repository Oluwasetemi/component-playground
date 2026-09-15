use crate::{StorySection, Theme};
use gpui::{div, prelude::*, px, size, App, Context, Hsla, IntoElement, Render, Window};
use gpui_component::{
    h_flex,
    input::{Input, InputState},
    list::{List, ListDelegate, ListItem, ListState},
    resizable::{h_resizable, resizable_panel},
    scroll::ScrollableElement as _,
    sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem},
    table::{
        Column, DataTable, Table, TableBody, TableCell, TableDelegate, TableHead, TableHeader,
        TableRow, TableState,
    },
    tree::{tree, TreeItem, TreeState},
    v_flex, v_virtual_list, ActiveTheme as _, IconName, IndexPath, VirtualListScrollHandle,
};
use std::{ops::Range, rc::Rc};

#[derive(Clone, Copy)]
pub enum DataLayoutKind {
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
}

struct DemoListDelegate {
    items: Vec<&'static str>,
    selected: Option<IndexPath>,
}

struct DemoTableDelegate;

impl TableDelegate for DemoTableDelegate {
    fn columns_count(&self, _: &App) -> usize {
        3
    }

    fn rows_count(&self, _: &App) -> usize {
        24
    }

    fn column(&self, ix: usize, _: &App) -> Column {
        match ix {
            0 => Column::new("id", "ID").width(px(100.)),
            1 => Column::new("name", "Name").width(px(220.)),
            _ => Column::new("status", "Status").width(px(140.)),
        }
    }

    fn render_td(
        &mut self,
        row: usize,
        col: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        match col {
            0 => format!("#{:04}", row + 1),
            1 => format!("Project {}", row + 1),
            _ => {
                if row.is_multiple_of(3) {
                    "Review".to_string()
                } else {
                    "Published".to_string()
                }
            }
        }
    }
}

impl ListDelegate for DemoListDelegate {
    type Item = ListItem;

    fn items_count(&self, _: usize, _: &App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        Some(ListItem::new(ix).child(self.items[ix.row]))
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) {
        self.selected = ix;
    }
}

pub struct DataLayoutStory {
    kind: DataLayoutKind,
    foreground: Hsla,
    list: gpui::Entity<ListState<DemoListDelegate>>,
    data_table: gpui::Entity<TableState<DemoTableDelegate>>,
    searchable_input: gpui::Entity<InputState>,
    tree: gpui::Entity<TreeState>,
    virtual_scroll: VirtualListScrollHandle,
}

impl DataLayoutStory {
    pub fn new(kind: DataLayoutKind, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let list = cx.new(|cx| {
            ListState::new(
                DemoListDelegate {
                    items: vec![
                        "Design system",
                        "Component playground",
                        "Release notes",
                        "Roadmap",
                    ],
                    selected: None,
                },
                window,
                cx,
            )
            .searchable(false)
        });
        let searchable_input = cx.new(|cx| InputState::new(window, cx).placeholder("Filter items"));
        let data_table = cx.new(|cx| TableState::new(DemoTableDelegate, window, cx));
        let tree = cx.new(|cx| {
            TreeState::new(cx).items(vec![
                TreeItem::new("src", "src").expanded(true).children([
                    TreeItem::new("components", "components")
                        .expanded(true)
                        .children([
                            TreeItem::new("button.rs", "button.rs"),
                            TreeItem::new("dialog.rs", "dialog.rs"),
                        ]),
                    TreeItem::new("lib.rs", "lib.rs"),
                ]),
                TreeItem::new("Cargo.toml", "Cargo.toml"),
            ])
        });

        Self {
            kind,
            foreground: Theme::groknight().foreground,
            list,
            data_table,
            searchable_input,
            tree,
            virtual_scroll: VirtualListScrollHandle::new(),
        }
    }

    fn shell(
        &self,
        title: &str,
        description: &str,
    ) -> gpui_component::scroll::Scrollable<gpui::Div> {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .text_color(self.foreground)
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child(title.to_string()))
                    .child(description.to_string()),
            )
    }

    fn panel(&self, content: impl IntoElement) -> impl IntoElement {
        div()
            .h(px(300.))
            .border_1()
            .rounded_md()
            .p_3()
            .text_color(self.foreground)
            .child(content)
    }
}

impl Render for DataLayoutStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.kind {
            DataLayoutKind::List => self
                .shell(
                    "List",
                    "Selectable, keyboard-friendly rows backed by ListState.",
                )
                .child(StorySection::new(
                    "Projects",
                    self.panel(List::new(&self.list)),
                )),
            DataLayoutKind::SearchableList => self
                .shell(
                    "Searchable List",
                    "A searchable list pattern using the available input and list primitives.",
                )
                .child(StorySection::new(
                    "Filter",
                    v_flex()
                        .gap_3()
                        .child(Input::new(&self.searchable_input))
                        .child(
                            self.panel(
                                v_flex().gap_2().children(
                                    ["Design system", "Component playground", "Release notes"]
                                        .into_iter()
                                        .map(|item| ListItem::new(item).child(item)),
                                ),
                            ),
                        ),
                )),
            DataLayoutKind::Table => self
                .shell(
                    "Table",
                    "Structured rows and columns with readable alignment and borders.",
                )
                .child(StorySection::new(
                    "Recent invoices",
                    self.panel(
                        Table::new()
                            .child(
                                TableHeader::new().child(
                                    TableRow::new()
                                        .child(TableHead::new().child("Invoice"))
                                        .child(TableHead::new().child("Status"))
                                        .child(TableHead::new().child("Amount")),
                                ),
                            )
                            .child(
                                TableBody::new().children([
                                    TableRow::new()
                                        .child(TableCell::new().child("INV-1042"))
                                        .child(TableCell::new().child("Paid"))
                                        .child(TableCell::new().child("$240.00")),
                                    TableRow::new()
                                        .child(TableCell::new().child("INV-1043"))
                                        .child(TableCell::new().child("Pending"))
                                        .child(TableCell::new().child("$180.00")),
                                    TableRow::new()
                                        .child(TableCell::new().child("INV-1044"))
                                        .child(TableCell::new().child("Paid"))
                                        .child(TableCell::new().child("$420.00")),
                                ]),
                            ),
                    ),
                )),
            DataLayoutKind::DataTable => self
                .shell(
                    "Data Table",
                    "Virtualized tabular data backed by a table delegate.",
                )
                .child(StorySection::new(
                    "Projects",
                    self.panel(DataTable::new(&self.data_table).stripe(true).bordered(true)),
                )),
            DataLayoutKind::Tree => self
                .shell(
                    "Tree",
                    "Expandable hierarchical data for files, folders, and nested resources.",
                )
                .child(StorySection::new(
                    "Project files",
                    self.panel(tree(&self.tree, |ix, entry, selected, _, _| {
                        ListItem::new(ix)
                            .selected(selected)
                            .child(entry.item().label.clone())
                    })),
                )),
            DataLayoutKind::VirtualList => {
                let sizes = Rc::new(vec![size(px(420.), px(32.)); 100]);
                self.shell(
                    "Virtual List",
                    "Only the visible rows are rendered for large collections.",
                )
                .child(StorySection::new(
                    "100 virtual rows",
                    self.panel(
                        v_virtual_list(
                            cx.entity(),
                            "virtual-rows",
                            sizes,
                            |_, range: Range<usize>, _, _| {
                                range
                                    .map(|ix| {
                                        div()
                                            .h(px(32.))
                                            .border_b_1()
                                            .border_color(gpui::rgba(0x2a2a2a66))
                                            .px_3()
                                            .flex()
                                            .items_center()
                                            .child(format!("Row {ix}"))
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(&self.virtual_scroll),
                    ),
                ))
            }
            DataLayoutKind::Scrollbar => {
                let rows =
                    (1..=24).map(|ix| div().h(px(28.)).child(format!("Scrollable row {ix}")));
                self.shell(
                    "Scrollbar",
                    "Explicit scrollbars keep long content discoverable.",
                )
                .child(StorySection::new(
                    "Scrollable content",
                    self.panel(div().h(px(240.)).overflow_y_scrollbar().children(rows)),
                ))
            }
            DataLayoutKind::ResizablePanels => self
                .shell(
                    "Resizable Panels",
                    "Drag the divider to adjust the workspace layout.",
                )
                .child(StorySection::new(
                    "Workspace",
                    self.panel(
                        h_resizable("data-layout-resizable")
                            .child(resizable_panel().size(px(180.)).child("Explorer"))
                            .child(resizable_panel().child(v_flex().p_4().child("Editor")))
                            .child(resizable_panel().size(px(220.)).child("Inspector")),
                    ),
                )),
            DataLayoutKind::Sidebar => self
                .shell(
                    "Sidebar",
                    "Grouped navigation with active and icon-bearing items.",
                )
                .child(StorySection::new(
                    "Navigation",
                    self.panel(
                        Sidebar::new("demo-sidebar")
                            .w(px(240.))
                            .header("Workspace")
                            .child(
                                SidebarGroup::new("Project").child(
                                    SidebarMenu::new()
                                        .child(
                                            SidebarMenuItem::new("Overview")
                                                .icon(IconName::LayoutDashboard)
                                                .active(true),
                                        )
                                        .child(SidebarMenuItem::new("Files").icon(IconName::File)),
                                ),
                            ),
                    ),
                )),
            DataLayoutKind::DockLayout => self
                .shell(
                    "Dock Layout",
                    "A dock-style workspace with persistent regions and tab-like panels.",
                )
                .child(StorySection::new(
                    "Workspace",
                    self.panel(
                        h_flex()
                            .h(px(260.))
                            .gap_2()
                            .child(
                                v_flex()
                                    .w(px(160.))
                                    .p_3()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .child("Files"),
                            )
                            .child(
                                v_flex()
                                    .flex_1()
                                    .p_3()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .child("Editor"),
                            )
                            .child(
                                v_flex()
                                    .w(px(180.))
                                    .p_3()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .child("Outline"),
                            ),
                    ),
                )),
        }
    }
}
