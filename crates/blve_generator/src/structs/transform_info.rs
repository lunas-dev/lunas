use crate::{orig_html_struct::structs::Node, transformers::utils::append_v_to_vars};

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
    pub action: EventTarget,
    pub target: String,
}

#[derive(Debug)]
pub struct NeededIdName {
    pub id_name: String,
    pub get_ref: bool,
    pub to_delete: bool,
    pub node_id: String,
}

#[derive(Debug)]
pub enum EventTarget {
    RefToFunction(String),
    Statement(String),
    EventBindingStatement(EventBindingStatement),
}

#[derive(Debug)]
pub struct EventBindingStatement {
    pub statement: String,
    pub arg: String,
}

impl ToString for EventTarget {
    fn to_string(&self) -> String {
        match self {
            EventTarget::RefToFunction(function_name) => function_name.clone(),
            EventTarget::Statement(statement) => format!("()=>{}", statement),
            EventTarget::EventBindingStatement(statement) => {
                format!("({})=>{}", statement.arg, statement.statement)
            }
        }
    }
}

impl EventTarget {
    pub fn new(content: String, variables: &Vec<String>) -> Self {
        // FIXME: This is a hacky way to check if the content is a statement or a function
        if content.trim().ends_with(")") {
            EventTarget::Statement(content)
        } else if word_is_one_word(content.as_str()) {
            EventTarget::RefToFunction(content)
        } else {
            EventTarget::Statement(append_v_to_vars(content.as_str(), &variables).0)
        }
    }
}

fn word_is_one_word(word: &str) -> bool {
    word.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

#[derive(Debug)]
pub struct IfBlockInfo {
    pub parent_id: String,
    pub target_if_blk_id: String,
    pub distance: u64,
    pub target_anchor_id: Option<String>,
    pub elm: Node,
    pub ref_text_node_id: Option<String>,
    pub condition: String,
    pub condition_dep_vars: Vec<String>,
    pub ctx: Vec<String>,
    pub if_block_id: String,
    pub element_location: Vec<usize>,
}

impl IfBlockInfo {
    pub fn generate_ctx_num(&self, if_blocks_infos: &Vec<IfBlockInfo>) -> usize {
        let mut ctx_num: u64 = 0;
        for (index, if_blk) in if_blocks_infos.iter().enumerate() {
            if self.ctx.contains(&if_blk.target_if_blk_id) {
                println!("match: {}", &if_blk.parent_id);
                let blk_num: u64 = (2 as u64).pow(index as u32);
                ctx_num = ctx_num | blk_num;
            }
        }

        ctx_num as usize
    }
}

pub fn sort_if_blocks(if_blocks: &mut Vec<IfBlockInfo>) {
    if_blocks.sort_by(|a, b| a.element_location.cmp(&b.element_location));
}

