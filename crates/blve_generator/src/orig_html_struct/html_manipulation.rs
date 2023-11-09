#[derive(Debug, Clone)]
pub struct HtmlManipulator {
    pub target_uuid: String,
    pub manipulations: HtmlManipulation,
}

#[derive(Debug, Clone)]
pub enum HtmlManipulation {
    RemoveChildForIfStatement(RemoveChildForIfStatement),
    SetIdForReactiveContent(SetIdForReactiveContent),
}

#[derive(Debug, Clone)]
pub struct RemoveChildForIfStatement {
    pub child_uuid: String,
    pub condition: String,
    pub block_id: String,
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct SetIdForReactiveContent {
    pub text: String,
    pub depenent_vars: Vec<String>,
    pub ctx: Vec<String>,
    pub elm_loc: Vec<usize>,
}
