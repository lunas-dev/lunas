mod transform_targets;

#[derive(Debug)]
pub struct AddStringToPosition {
    pub position: u32,
    pub string: String,
}

#[derive(Debug)]
pub struct VariableNameAndAssignedNumber {
    pub name: String,
    pub assignment: u32,
}

#[derive(Debug)]
pub struct ActionAndTarget {
    pub action_name: String,
    pub action: String,
    pub target: String,
}

#[derive(Debug)]
pub struct NeededIdName {
    pub id_name: String,
    pub to_delete: bool,
}

pub enum ElmAndReactiveInfo {
    ElmAndVariableRelation(ElmAndVariableContentRelation),
    ElmAndReactiveAttributeRelation(ElmAndReactiveAttributeRelation),
}

#[derive(Debug)]
pub struct ElmAndVariableContentRelation {
    pub elm_id: String,
    pub variable_names: Vec<String>,
    pub content_of_element: String,
}

#[derive(Debug)]
pub struct ElmAndReactiveAttributeRelation {
    pub elm_id: String,
    pub reactive_attr: Vec<ReactiveAttr>,
}

#[derive(Debug)]
pub struct ReactiveAttr {
    pub attribute_key: String,
    pub content_of_attr: String,
    pub variable_names: Vec<String>,
}
