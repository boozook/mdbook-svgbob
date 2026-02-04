extern crate svg;
extern crate svgbob;
extern crate toml;

use svg::node::element::Style;
use svgbob::CellBuffer;
use svgbob::Node;
pub use svgbob::Settings;

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
