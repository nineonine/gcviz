#[derive(Debug)]
pub struct GCStats;

impl GCStats {
    pub fn new() -> Self {
        GCStats {}
    }
}

impl Default for GCStats {
    fn default() -> Self {
        Self::new()
    }
}
