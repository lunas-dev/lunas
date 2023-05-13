use html_parser::Dom;
use rome_js_syntax::AnyJsRoot;

#[derive(Debug)]
pub struct DetailedLanguageBlocks {
    pub dom: Dom,
    pub css: Option<String>,
    pub js: Option<AnyJsRoot>,
}

