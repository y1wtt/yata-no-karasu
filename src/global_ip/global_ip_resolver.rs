use std::error::Error;
pub(crate) trait GlobalIpResolver {
    fn new() -> Self;
    async fn get_current_ip(&self) -> Result<String, Box<dyn Error>>;
}