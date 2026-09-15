use gpui::{div, prelude::*, px, App, ClickEvent, ElementId, IntoElement, SharedString, Window};
use gpui_component::{
    sidebar::{SidebarItem, SidebarMenuItem},
    v_flex, ActiveTheme as _, Collapsible, Icon, IconName,
};
use std::rc::Rc;

type ToggleHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

#[derive(Clone)]
pub struct SidebarStoryGroup {
    label: SharedString,
    icon: IconName,
    children: Vec<SidebarMenuItem>,
    description: Option<SharedString>,
    collapsed: bool,
    open: bool,
    on_toggle: ToggleHandler,
}

impl SidebarStoryGroup {
    pub fn new(
        label: impl Into<SharedString>,
        icon: IconName,
        children: impl IntoIterator<Item = SidebarMenuItem>,
    ) -> Self {
        Self {
            label: label.into(),
            icon,
            children: children.into_iter().collect(),
            description: None,
            collapsed: false,
            open: false,
            on_toggle: Rc::new(|_, _, _| {}),
        }
    }

    pub fn empty_state(
        label: impl Into<SharedString>,
        description: impl Into<SharedString>,
        icon: IconName,
    ) -> Self {
        Self {
            label: label.into(),
            icon,
            children: Vec::new(),
            description: Some(description.into()),
            collapsed: false,
            open: false,
            on_toggle: Rc::new(|_, _, _| {}),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Rc::new(handler);
        self
    }
}

impl Collapsible for SidebarStoryGroup {
    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

impl SidebarItem for SidebarStoryGroup {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let id = id.into();
        let open = !self.collapsed && self.open;
        let label = self.label.clone();
        let icon = self.icon;
        let collapsed = self.collapsed;
        let children = self.children;
        let description = self.description;
        let on_toggle = self.on_toggle;

        if let Some(description) = description {
            return v_flex()
                .items_center()
                .gap_2()
                .px_3()
                .py_6()
                .text_center()
                .text_color(cx.theme().muted_foreground)
                .child(Icon::new(icon).size_5())
                .child(div().text_color(cx.theme().foreground).child(label))
                .child(div().text_size(px(12.)).child(description))
                .into_any_element();
        }

        v_flex()
            .child(
                SidebarMenuItem::new(label)
                    .icon(icon)
                    .collapsed(collapsed)
                    .suffix(move |_, _| {
                        Icon::new(if open {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .size_4()
                    })
                    .on_click(move |event, window, cx| on_toggle(event, window, cx))
                    .render(format!("{id}-summary"), window, cx),
            )
            .when(open, |this| {
                this.child(
                    v_flex()
                        .id("sidebar-story-group-items")
                        .border_l_1()
                        .border_color(cx.theme().sidebar_border)
                        .gap_1()
                        .ml_3p5()
                        .pl_2p5()
                        .py_0p5()
                        .children(children.into_iter().enumerate().map(|(index, item)| {
                            item.collapsed(collapsed)
                                .render(format!("{id}-item-{index}"), window, cx)
                                .into_any_element()
                        })),
                )
            })
            .into_any_element()
    }
}
