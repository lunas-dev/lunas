#[derive(Debug, Clone)]
pub struct HtmlManipulator {
    pub target_uuid: String,
    pub manipulations: HtmlManipulation,
}

#[derive(Debug, Clone)]
pub enum HtmlManipulation {
    RemoveChildForIfStatement(RemoveChildForIfStatement),
    SetIdForReactiveContent(SetIdToParentForChildReactiveText),
    RemoveChildTextNode(RemoveChildTextNode),
}

#[derive(Debug, Clone)]
pub struct RemoveChildForIfStatement {
    pub child_uuid: String,
    pub condition: String,
    pub block_id: String,
    // TODO:ctxとlocをHtmlManipulatorに入れるか検討する
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct SetIdToParentForChildReactiveText {
    pub text: String,
    pub depenent_vars: Vec<String>,
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct RemoveChildTextNode {
    pub depenent_vars: Vec<String>,
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
    pub child_uuid: String,
    pub content: String,
}
