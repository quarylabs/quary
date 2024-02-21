use quary_proto::Test;

#[derive(Debug, Clone, PartialEq)]
pub struct Inference {
    pub test: Test,
    pub action: InferenceTestRunnerAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InferenceTestRunnerAction {
    Run,
    SkipBecauseInferredFromTest(String),
    SkipBecauseInferredFromTestThroughOperation { test: String, operation: String },
    SkipBecauseCountStar,
}
