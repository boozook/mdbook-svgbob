extern crate toml;
extern crate svg;
extern crate svgbob;

use svgbob::{CellBuffer, Node};
pub use svgbob::Settings;
use svg::node::element::Style;

/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
	let cb = CellBuffer::from(s);
	let (svg, _, height): (Node<()>, f32, f32) = cb.get_node_with_size(settings);

	let mut source = String::new();
	svg.render_with_indent(&mut source, 0, true).expect("html render");

	let style = Style::new("svg { width: 100% !important; }").set("type", "text/css");
	format!("<div style='width:100%; height:{}px; fill:var(--fg);'>{}{}</div>", height, style, source).replace('\n', "")
}
