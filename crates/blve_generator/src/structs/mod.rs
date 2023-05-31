mod transform_targets;

#[derive(Debug)]
pub struct AddDotV {
    pub position: u32,
}

#[derive(Debug)]
pub enum TransformAnalysisResult {
    AddDotV(AddDotV),
}

#[derive(Debug)]
pub struct AddStringToPosition {
    pub position: u32,
    pub string: String,
}

pub type TransformAnalysisResults = Vec<TransformAnalysisResult>;
