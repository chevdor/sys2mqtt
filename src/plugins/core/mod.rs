use async_trait::async_trait;
use rumqttc::AsyncClient;

#[async_trait]
pub trait Plugin {
    // Name of the plugin = name of the module by default
    fn name(&self) -> &str;

    fn is_enabled(&self) -> bool;  // Check if the plugin is enabled

    async fn start(&self, client: &AsyncClient);
}
