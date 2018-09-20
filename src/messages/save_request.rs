#[derive(Message)]
pub struct SaveRequest;

impl SaveRequest {
    fn new() -> Self {
        Self {}
    }
}

impl Default for SaveRequest {
    fn default() -> Self {
        Self::new()
    }
}
