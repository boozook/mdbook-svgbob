pub use svgbob::Settings;
use serde::Deserialize;
use toml::value::{Value, Table};


pub const DEFAULT_CODE_BLOCK: &str = "bob";
pub const DEFAULT_BACKGROUND: &str = "transparent";
pub const DEFAULT_STROKE_COLOR: &str = "var(--fg)";


#[derive(Debug, Deserialize)]
pub struct Cfg {
	#[serde(flatten)]
	#[serde(with = "SettingsDe")]
	pub settings: Settings,

	#[serde(alias = "code-block")]
	#[serde(default = "Cfg::default_code_block")]
	pub code_block: String,
}

#[derive(Debug, Deserialize)]
#[serde(remote = "svgbob::Settings")]
#[serde(default)]
struct SettingsDe {
	#[serde(alias = "font-size")]
	pub font_size: usize,
	#[serde(alias = "font-family")]
	pub font_family: String,
	#[serde(alias = "fill-color")]
	pub fill_color: String,
	#[serde(default = "SettingsDe::default_background")]
	pub background: String,
	#[serde(alias = "stroke-color")]
	#[serde(default = "SettingsDe::default_stroke_color")]
	pub stroke_color: String,
	#[serde(alias = "stroke-width")]
	pub stroke_width: f32,
	pub scale: f32,
	#[serde(alias = "enhance-circuitries")]
	pub enhance_circuitries: bool,
	#[serde(alias = "include-backdrop")]
	pub include_backdrop: bool,
	#[serde(alias = "include-styles")]
	pub include_styles: bool,
	#[serde(alias = "include-defs")]
	pub include_defs: bool,
	#[serde(alias = "merge-line-with-shapes")]
	pub merge_line_with_shapes: bool,
}

impl Default for SettingsDe {
	fn default() -> Self { Settings::default().into() }
}

impl SettingsDe {
	#[inline]
	fn default_background() -> String { DEFAULT_BACKGROUND.into() }

	#[inline]
	fn default_stroke_color() -> String { DEFAULT_STROKE_COLOR.into() }
}

impl From<Settings> for SettingsDe {
	fn from(settings: Settings) -> Self {
		let Settings { font_size,
		               font_family,
		               fill_color,
		               background,
		               stroke_color,
		               stroke_width,
		               scale,
		               enhance_circuitries,
		               include_backdrop,
		               include_styles,
		               include_defs,
		               merge_line_with_shapes,
		               .. } = settings;
		Self { font_size,
		       font_family,
		       fill_color,
		       background,
		       stroke_color,
		       stroke_width,
		       scale,
		       enhance_circuitries,
		       include_backdrop,
		       include_styles,
		       include_defs,
		       merge_line_with_shapes }
	}
}


impl Cfg {
	#[inline]
	fn default_code_block() -> String { DEFAULT_CODE_BLOCK.into() }
}

impl Default for Cfg {
	// This using defaults defined above for serde, just to not repeat.
	fn default() -> Self {
		Value::from(Table::new()).try_into()
		                         .expect("empty table with serde-defined default values")
	}
}

impl TryFrom<Table> for Cfg {
	type Error = toml::de::Error;

	fn try_from(map: Table) -> Result<Self, Self::Error> {
		let value: Value = map.into();
		value.try_into()
	}
}

impl TryFrom<&'_ Table> for Cfg {
	type Error = toml::de::Error;

	fn try_from(map: &'_ Table) -> Result<Self, Self::Error> {
		let value: Value = map.to_owned().into();
		value.try_into()
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deserialize_with_defaults() {
		let source: Table = [
		                     (String::from("include-styles"), true.into()),
		                     (String::from("scale"), 42.0.into()),
		                     (String::from("code-block"), "anything".into()),
		].into_iter()
		                    .collect();

		let cfg = Cfg::try_from(source).unwrap();

		assert!(cfg.settings.include_styles);
		assert_eq!(cfg.settings.scale, 42.0);
		assert_eq!(cfg.settings.background, "transparent");
		assert_eq!(cfg.code_block, "anything");
	}

	#[test]
	// Just should not fail.
	fn cfg_default() { Cfg::default(); }
}
