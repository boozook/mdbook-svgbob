<<<<<<< HEAD
=======
extern crate toml;
extern crate svg;
extern crate svgbob;

use svgbob::CellBuffer;
use svgbob::Node;

pub use svgbob::Settings;
use svg::node::element::Style;
>>>>>>> master
use serde::Deserialize;
use svg::node::element::Style;
pub use svgbob::Settings;
use svgbob::{CellBuffer, Node, Render};
use toml::{map::Map, value::Value};

type CfgMap = Map<String, Value>;

pub fn cfg_to_settings(cfg: &CfgMap) -> Settings {
<<<<<<< HEAD
    Settings {
        font_size: cfg_prop_or(cfg, "font_size", 14),
        font_family: cfg_prop_or(cfg, "font_family", "arial".to_owned()),
        fill_color: cfg_prop_or(cfg, "fill_color", "black".to_owned()),
        background: cfg_prop_or(cfg, "background", "transparent".to_owned()),
        stroke_color: cfg_prop_or(cfg, "stroke_color", "var(--fg)".to_owned()),
        stroke_width: cfg_prop_or(cfg, "stroke_width", 2.0),
        scale: cfg_prop_or(cfg, "scale", 8.0),
        enhance_circuitries: cfg_prop_or(cfg, "enhance_circuitries", true),
        include_backdrop: cfg_prop_or(cfg, "include_backdrop", true),
        include_styles: cfg_prop_or(cfg, "include_styles", true),
        include_defs: cfg_prop_or(cfg, "include_defs", true),
        merge_line_with_shapes: cfg_prop_or(cfg, "merge_line_with_shapes", true),
    }
=======
	let mut settings = Settings::default();
	// settings.class = cfg_prop_or(&cfg, "class", Some("bob".to_owned()));
	// settings.text_width = cfg_prop_or(&cfg, "text_width", 8.0);
	// settings.text_height = cfg_prop_or(&cfg, "text_height", 16.0);
	// settings.scale = cfg_prop_or(&cfg, key:"scale", def:"1".to_owned());
	settings.font_family = cfg_prop_or(&cfg, "font_family", "arial".to_owned());
	settings.font_size = cfg_prop_or(&cfg, "font_size", 14);
	settings.stroke_color = cfg_prop_or(&cfg, "stroke_color", "var(--fg)".to_owned());
	settings.stroke_width = cfg_prop_or(&cfg, "stroke_width", 2.0);
	settings.background = cfg_prop_or(&cfg, "background_color", "transparent".to_owned());
	settings
>>>>>>> master
}

fn cfg_prop_or<'de, T: Deserialize<'de>>(cfg: &CfgMap, key: &str, def: T) -> T {
    cfg.get(key)
        .map(|v| {
            v.clone()
                .try_into()
                .map_err(|err| log::error!("{}", err))
                .ok()
        })
        .flatten()
        .unwrap_or(def)
}

/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
<<<<<<< HEAD
<<<<<<< HEAD
    let cb = CellBuffer::from(s);
    let (node, _width, height): (Node<()>, f32, f32) = cb.get_node_with_size(settings);
    let mut svg = String::new();
    node.render(&mut svg).expect("failed to render bob diagram");
    let style = Style::new("svg { width: 100% !important; }").set("type", "text/css");
    format!(
        "<div style='width:100%; height:{}px;'>{}{}</div>",
        height, style, svg
    )
    .replace("\n", "")
=======
=======
>>>>>>> 1bbd1ff (	modified:   Cargo.toml)
	let cb = CellBuffer::from(s);
    let (_node, _w, height): (Node<()>, f32, f32) = cb.get_node_with_size(&settings);
	let svg = svgbob::to_svg_with_settings(s, &settings);
	let style = Style::new("svg { width: 100% !important; }").set("type", "text/css");
	
	format!("<div style='width:100%; height:{}px;'>{}{}</div>", height, style, svg).replace("\n", "")
>>>>>>> master
}
