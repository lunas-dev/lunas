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

// TODO: contentを追加
#[derive(Debug)]
pub struct ElmAndVariableRelation {
    pub elm_id: String,
    pub variable_names: Vec<String>,
}
