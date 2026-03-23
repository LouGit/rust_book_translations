// A preprocessor mdBook, so as to treat the data flow to translate, when
// necessary, items contained inside code blocks.
//
// // IMPORTANT:
// Translation is done on a per-line exact match basis.
// This means that:
//   - trailing spaces
//   - indentation
//   - prompt symbols ($)
// may fail to match if not normalised.



use mdbook_driver::book::{Book, BookItem};
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;

/// CodeTranslator is a mdBook preprocessor that translates lines
/// inside code blocks using gettext `.po` files.
///
/// Implementation notes:
/// - We precompute a HashMap<msgid, msgstr>
///   to avoid scanning the full catalog for each line
///   (catalog can be large for the Rust book)
/// - Matching is exact (no normalisation)
///   -> faster, but sensitive to whitespace differences
pub struct CodeTranslator {
    // We precompute a HashMap to avoid O(n*m) lookup when
    // translating each line of code blocks (catalog can be
    // large for the Rust book).
    translations: HashMap<String, String>,
}

impl CodeTranslator{
    pub fn new(po_file: &Path) -> Result<Self> {
        let catalog = polib::po_file::parse(po_file)?;

        let mut translations = HashMap::new();

        for message in catalog.messages() {
            if let Ok(msgstr) = message.msgstr() {
                translations.insert(
                    message.msgid().to_string(),
                    msgstr.to_string(),
                );
            }
        }
        Ok(Self {translations})
    }

    fn translate_line(&self, line: &str) -> String {
    // Exact match lookup:
    // we deliberately avoid trimming or normalizing,
    // because `.po` entries must match the original source exactly.
        self.translations
            .get(line)
            .cloned()
            .unwrap_or_else(|| line.to_string())
    }
}

impl Preprocessor for CodeTranslator {
    fn name(&self) -> &str {
        "code-translator"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        for item in &mut book.items {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = crate::listings::process_code_blocks(
                    &chapter.content,
                    |line| {
                        self.translate_line(line)
                });
            };
        }
        Ok(book)
    }

}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fn 
}
*/
