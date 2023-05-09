use html_parser::Dom;
use rome_js_syntax::AnyJsRoot;
// use rome_js_parser::

#[derive(Debug)]
pub struct DetailedLanguageBlocks {
    pub dom: Dom,
    pub css: Option<String>,
    pub js: Option<AnyJsRoot>,
}

// parsejs
