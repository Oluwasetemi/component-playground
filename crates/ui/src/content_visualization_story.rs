use crate::{StorySection, Theme};
use gpui::{div, prelude::*, px, Context, Hsla, IntoElement, Render, SharedString, Window};
use gpui_component::{
    chart::{AreaChart, BarChart, CandlestickChart, LineChart, PieChart, RadarChart, SankeyChart},
    highlighter::SyntaxHighlighter,
    plot::shape::SankeyLink,
    scroll::ScrollableElement as _,
    tab::{Tab, TabBar},
    text::{self, TextView},
    v_flex, ActiveTheme as _, Icon, IconName,
};

#[derive(Clone, Copy)]
pub enum ContentVisualizationKind {
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
}

#[derive(Clone)]
struct Point {
    label: SharedString,
    value: f64,
}

#[derive(Clone)]
struct Candle {
    date: SharedString,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[derive(Clone)]
struct FlowNode {
    name: SharedString,
}

pub struct ContentVisualizationStory {
    kind: ContentVisualizationKind,
    foreground: Hsla,
    surface: Hsla,
    syntax_language: usize,
    points: Vec<Point>,
    candles: Vec<Candle>,
    flow_nodes: Vec<FlowNode>,
    flow_links: Vec<SankeyLink>,
}

impl ContentVisualizationStory {
    pub fn new(kind: ContentVisualizationKind, _: &mut Window, _: &mut Context<Self>) -> Self {
        let points = [
            ("Jan", 120.),
            ("Feb", 180.),
            ("Mar", 150.),
            ("Apr", 230.),
            ("May", 205.),
            ("Jun", 280.),
        ]
        .into_iter()
        .map(|(label, value)| Point {
            label: label.into(),
            value,
        })
        .collect();
        let candles = [
            ("Mon", 120., 142., 110., 136.),
            ("Tue", 136., 150., 126., 130.),
            ("Wed", 130., 158., 124., 151.),
            ("Thu", 151., 166., 145., 160.),
            ("Fri", 160., 172., 150., 168.),
        ]
        .into_iter()
        .map(|(date, open, high, low, close)| Candle {
            date: date.into(),
            open,
            high,
            low,
            close,
        })
        .collect();
        let flow_nodes = ["Visitors", "Sign ups", "Trials", "Customers"]
            .into_iter()
            .map(|name| FlowNode { name: name.into() })
            .collect();
        let flow_links = vec![
            SankeyLink::new(0, 1, 720.),
            SankeyLink::new(1, 2, 420.),
            SankeyLink::new(2, 3, 180.),
        ];

        Self {
            kind,
            foreground: Theme::groknight().foreground,
            surface: Theme::groknight().surface_raised,
            syntax_language: 0,
            points,
            candles,
            flow_nodes,
            flow_links,
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

    fn chart_frame(&self, title: &str, chart: impl IntoElement) -> impl IntoElement {
        v_flex()
            .h(px(320.))
            .gap_2()
            .border_1()
            .border_color(gpui::rgba(0x2a2a2a99))
            .rounded_md()
            .p_4()
            .bg(self.surface)
            .text_color(self.foreground)
            .child(title.to_string())
            .child(div().flex_1().child(chart))
    }
}

impl Render for ContentVisualizationStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.kind {
            ContentVisualizationKind::Icon => self
                .shell("Icon", "Scalable symbols from the component icon set.")
                .child(StorySection::new(
                    "Icon set",
                    gpui_component::h_flex()
                        .gap_6()
                        .children([
                            (IconName::LayoutDashboard, "Home"),
                            (IconName::Search, "Search"),
                            (IconName::Bell, "Alerts"),
                            (IconName::Settings2, "Settings"),
                        ].into_iter().map(|(icon, label)| v_flex().items_center().gap_2().child(Icon::new(icon).size_6()).child(label))),
                )),
            ContentVisualizationKind::TextView => self
                .shell("Text View", "Rich text rendered through the component text view.")
                .child(StorySection::new(
                    "Rendered text",
                    TextView::markdown("text-view", "# Text View\n\nTextView renders **formatted content** with consistent typography."),
                )),
            ContentVisualizationKind::Markdown => self
                .shell("Markdown", "Markdown content rendered with the built-in text pipeline.")
                .child(StorySection::new(
                    "Markdown document",
                    text::markdown("## Release notes\n\n- Faster navigation\n- Better keyboard support\n- Improved visual contrast"),
                )),
            ContentVisualizationKind::HtmlRendering => self
                .shell("HTML Rendering", "HTML content rendered through the supported text API.")
                .child(StorySection::new(
                    "HTML fragment",
                    text::html("<h3>Embedded content</h3><p>This fragment is rendered by the HTML text pipeline.</p>"),
                )),
            ContentVisualizationKind::SyntaxHighlighter => {
                let languages = [
                    ("Rust", "rust"),
                    ("TypeScript", "typescript"),
                    ("Python", "python"),
                    ("Go", "go"),
                    ("JSON", "json"),
                    ("HTML", "html"),
                    ("CSS", "css"),
                ];
                let samples = [
                    "fn main() {\n    println!(\"Hello, GPUI\");\n}",
                    "const greeting: string = \"Hello, GPUI\";\nconsole.log(greeting);",
                    "def greet(name: str) -> str:\n    return f\"Hello, {name}\"",
                    "func greet(name string) string {\n    return \"Hello, \" + name\n}",
                    "{\n  \"name\": \"component-playground\",\n  \"version\": \"0.1.0\"\n}",
                    "<main class=\"hero\">\n  <h1>Hello, GPUI</h1>\n</main>",
                    ".hero {\n  color: #7aa2f7;\n  padding: 1rem;\n}",
                ];
                let index = self.syntax_language.min(languages.len() - 1);
                let (label, language) = languages[index];
                let highlighter = SyntaxHighlighter::new(language);
                let code = format!("```{language}\n{}\n```", samples[index]);

                self.shell(
                    "Syntax Highlighter",
                    "Switch languages to compare syntax-aware code presentation.",
                )
                .child(StorySection::new(
                    "Languages",
                    TabBar::new("syntax-languages")
                        .w_full()
                        .menu(true)
                        .selected_index(index)
                        .on_click(cx.listener(|this, selected, _, cx| {
                            this.syntax_language = *selected;
                            cx.notify();
                        }))
                        .children(languages.into_iter().map(|(label, _)| Tab::new().label(label))),
                ))
                .child(StorySection::new(
                    format!("{label} ({})", highlighter.language()),
                    text::markdown(code),
                ))
            }
            ContentVisualizationKind::LineChart => self
                .shell("Line Chart", "Trends over a continuous sequence.")
                .child(StorySection::new("Monthly visitors", self.chart_frame("Visitors", LineChart::new(self.points.clone()).x(|point| point.label.clone()).y(|point| point.value).name("Visitors")))),
            ContentVisualizationKind::BarChart => self
                .shell("Bar Chart", "Discrete comparisons across categories.")
                .child(StorySection::new("Monthly visitors", self.chart_frame("Visitors", BarChart::new(self.points.clone()).band(|point| point.label.clone()).value(|point| point.value).name("Visitors")))),
            ContentVisualizationKind::AreaChart => self
                .shell("Area Chart", "Trend data with a filled visual surface.")
                .child(StorySection::new("Monthly visitors", self.chart_frame("Visitors", AreaChart::new(self.points.clone()).x(|point| point.label.clone()).y(|point| point.value).name("Visitors")))),
            ContentVisualizationKind::PieChart => self
                .shell("Pie Chart", "Part-to-whole distribution across categories.")
                .child(StorySection::new(
                    "Traffic sources",
                    self.chart_frame(
                        "Sources",
                        PieChart::new(self.points.clone())
                            .value(|point| point.value as f32)
                            .color({
                                let theme = Theme::groknight();
                                move |point| match point.label.as_ref() {
                                    "Jan" => theme.blue,
                                    "Feb" => theme.cyan,
                                    "Mar" => theme.magenta,
                                    "Apr" => theme.green,
                                    "May" => theme.yellow,
                                    _ => theme.red,
                                }
                            })
                            .label(|point| point.label.clone())
                            .label_color(self.foreground)
                            .label_line_color({
                                let theme = Theme::groknight();
                                move |_| theme.muted_strong
                            })
                            .outer_radius(92.),
                    ),
                )),
            ContentVisualizationKind::CandlestickChart => self
                .shell("Candlestick Chart", "Open, high, low, and close values for market data.")
                .child(StorySection::new("Weekly price movement", self.chart_frame("Price", CandlestickChart::new(self.candles.clone()).x(|candle| candle.date.clone()).open(|candle| candle.open).high(|candle| candle.high).low(|candle| candle.low).close(|candle| candle.close)))),
            ContentVisualizationKind::RadarChart => self
                .shell("Radar Chart", "Compare multiple dimensions around a shared scale.")
                .child(StorySection::new("Team capabilities", self.chart_frame("Capabilities", RadarChart::new(self.points.clone()).label(|point| point.label.clone()).value(|point| point.value).name("Score")))),
            ContentVisualizationKind::SankeyChart => self
                .shell("Sankey Chart", "Flow volume between connected stages.")
                .child(StorySection::new("Conversion flow", self.chart_frame("Funnel", SankeyChart::new(self.flow_nodes.clone(), self.flow_links.clone()).node_label(|node| node.name.clone())))),
            ContentVisualizationKind::Plot => self
                .shell("Plot", "A coordinate-space visualization surface for custom drawing.")
                .child(StorySection::new(
                    "Plot surface",
                    div()
                        .h(px(260.))
                        .border_1()
                        .border_color(cx.theme().border)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child("Custom Plot drawing surface"),
                )),
        }
    }
}
