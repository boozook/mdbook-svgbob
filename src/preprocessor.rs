extern crate semver;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use mdbook::errors::Error;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};

use crate::Result;
use crate::svgbob::*;
use crate::cfg::Cfg;


/// Svgbob preprocessor for mdbook.
pub struct Bob;

impl Bob {
	pub fn new() -> Self { Self }

	#[allow(dead_code)]
	pub fn handle_preprocessing(&self) -> Result {
		use std::io::{stdin, stdout};
		use semver::{Version, VersionReq};

		let (ctx, book) = CmdPreprocessor::parse_input(stdin())?;
		let current = Version::parse(&ctx.mdbook_version)?;
		let built = VersionReq::parse(&format!("~{}", mdbook::MDBOOK_VERSION))?;

		if ctx.mdbook_version != mdbook::MDBOOK_VERSION && !built.matches(&current) {
			warn!(
			      "The {} plugin was built against version {} of mdbook, \
				      but we're being called from version {}, so may be incompatible.",
			      self.name(),
			      mdbook::MDBOOK_VERSION,
			      ctx.mdbook_version
			);
		}
		let processed_book = self.run(&ctx, book)?;
		serde_json::to_writer(stdout(), &processed_book)?;
		Ok(())
	}
}


impl Preprocessor for Bob {
	fn name(&self) -> &str { "svgbob" }

	// Any possible renderer is supported because we're including svg into md source, so it all on md-renderer.
	// It can be just html or any other such as epub or pdf.
	// TODO: Maybe declare list of known supported renderers?
	fn supports_renderer(&self, renderer: &str) -> bool { renderer != "not-supported" }


	fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
		let cfg: Cfg = {
			               ctx.config
			                  .get_preprocessor(self.name())
			                  .and_then(|map| map.try_into().map_err(|err| error!("{}", err)).ok())
		               }.unwrap_or_default();

		book.for_each_mut(|item| {
			    if let BookItem::Chapter(chapter) = item {
				    let _ = process_code_blocks(chapter, &cfg).map(|s| {
					                                              chapter.content = s;
					                                              trace!("chapter '{}' processed", &chapter.name);
				                                              })
				                                              .map_err(|err| {
					                                              error!("{}", err);
				                                              });
			    }
		    });
		Ok(book)
	}
}

/// Find code-blocks \`\`\`bob, produce svg and place it instead code.
fn process_code_blocks(chapter: &mut Chapter, cfg: &Cfg) -> Result<String, std::fmt::Error> {
	use pulldown_cmark::{CodeBlockKind, Event, CowStr, Tag, TagEnd};
	use pulldown_cmark_to_cmark::cmark;

	enum State {
		None,
		Open,
		Closing,
	}

	let mut state = State::None;
	let mut buf = String::with_capacity(chapter.content.len());
	// The curly_quotes setting is left at false so that people can
	// set it in book.toml (mdBook will apply the setting when it
	// parses our output). It is important to use new_cmark_parser so
	// that we parse things like tables consistently with mdBook.
	let parser = mdbook::utils::new_cmark_parser(&chapter.content, false);
	// Clippy false-positive issue:
	// https://github.com/rust-lang/rust-clippy/issues/9211#issuecomment-1335173323
	#[allow(clippy::unnecessary_filter_map)]
	let events = parser.filter_map(|e| {
		                 use State::*;
		                 use Event::*;
		                 use CowStr::*;
		                 use CodeBlockKind::*;
		                 use Tag::{CodeBlock, Paragraph};

		                 match (&e, &mut state) {
			                 (Start(CodeBlock(Fenced(Borrowed(mark)))), None) if mark == &cfg.code_block => {
			                    state = Open;
			                    Some(Start(Paragraph))
		                    },

		                    (Text(Borrowed(text)), Open) => {
			                    state = Closing;
			                    Some(Html(bob_handler(text, &cfg.settings).into()))
		                    },

		                    (End(TagEnd::CodeBlock), Closing) => {
			                    state = None;
			                    Some(End(TagEnd::Paragraph))
		                    },
		                    _ => Some(e),
		                 }
	                 });
	cmark(events, &mut buf).map(|_| buf)
}

#[cfg(test)]
mod tests {

	#[test]
	fn process_code_blocks() {
		use super::{Cfg, Chapter, process_code_blocks};

		let settings = Cfg::default();
		let mut chapter = Chapter::new("test", "```bob\n-->\n```".to_owned(), ".", Vec::with_capacity(0));
		let result = process_code_blocks(&mut chapter, &settings).unwrap();
		assert!(result.contains("<svg"));
		assert!(result.contains("<line"));
		assert!(result.contains("#arrow"));
	}
}
