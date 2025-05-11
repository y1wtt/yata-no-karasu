use std::error::Error;
use std::fmt::Display;
pub(crate) trait GlobalIpResolver {
    fn new() -> Self;
    async fn get_current_ip(&self) -> Result<String, Box<dyn Error>>;
}