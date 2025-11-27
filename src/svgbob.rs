use svg::node::element::Style;
pub use svgbob::Settings;
use svgbob::{CellBuffer, Node};

/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
    let cb = CellBuffer::from(s);
    let (svg, _, height): (Node<()>, f32, f32) = cb.get_node_with_size(settings);

    let mut source = String::new();
    svg.render_with_indent(&mut source, 0, true)
        .expect("html render");

    let style = Style::new("svg { width: 100% !important; }").set("type", "text/css");
    format!(
        "<div style='width:100%; height:{}px;'>{}{}</div>",
        height, style, source
    )
    .replace('\n', "")
}
