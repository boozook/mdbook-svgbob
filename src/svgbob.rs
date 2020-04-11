extern crate toml;
extern crate svg;
extern crate svgbob;

use svgbob::Grid;
pub use svgbob::Settings;
use svg::node::element::Style;
use serde::Deserialize;
use toml::{map::Map, value::Value};


type CfgMap = Map<String, Value>;


pub fn cfg_to_settings(cfg: &CfgMap) -> Settings {
	let mut settings = Settings::default();
	settings.class = cfg_prop_or(&cfg, "class", Some("bob".to_owned()));
	settings.text_width = cfg_prop_or(&cfg, "text_width", 8.0);
	settings.text_height = cfg_prop_or(&cfg, "text_height", 16.0);
	settings.font_family = cfg_prop_or(&cfg, "font_family", "arial".to_owned());
	settings.font_size = cfg_prop_or(&cfg, "font_size", 14.0);
	settings.stroke_color = cfg_prop_or(&cfg, "stroke_color", "var(--fg)".to_owned());
	settings.stroke_width = cfg_prop_or(&cfg, "stroke_width", 2.0);
	settings.background_color = cfg_prop_or(&cfg, "background_color", "transparent".to_owned());
	settings
}

fn cfg_prop_or<'de, T: Deserialize<'de>>(cfg: &CfgMap, key: &str, def: T) -> T {
	cfg.get(key)
	   .map(|v| v.clone().try_into().map_err(|err| error!("{}", err)).ok())
	   .flatten()
	   .unwrap_or(def)
}


/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
	let grid = Grid::from_str(s, &settings);
	let svg = grid.get_svg();
	let (_, height) = grid.get_size();
	let style = Style::new("svg { width: 100% !important; }").set("type", "text/css");
	format!("<div style='width:100%; height:{}px;'>{}{}</div>", height, style, svg).replace("\n", "")
}
