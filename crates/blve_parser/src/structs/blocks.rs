use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct LanguageBlock {
    pub language_name: String,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct MetaData {
    pub kind: String,
    pub params: HashMap<String, String>,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub enum ParsedItem {
    LanguageBlock(LanguageBlock),
    MetaData(MetaData),
}
