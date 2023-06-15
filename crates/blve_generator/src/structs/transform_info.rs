#[derive(Debug, Clone)]
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
