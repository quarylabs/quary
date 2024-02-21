#[derive(Default)]
pub struct ConvertDbtStats {
    pub models_created: usize,
    pub tests_created: usize,
    pub file_tests_created: usize,
    pub seeds_created: usize,
    pub errors: Vec<String>,
}

impl ConvertDbtStats {
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}
