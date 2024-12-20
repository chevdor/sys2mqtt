use async_trait::async_trait;
use rumqttc::AsyncClient;

#[async_trait]
pub trait Plugin {
    fn name(&self) -> &str;  // Name of the plugin
    fn is_enabled(&self) -> bool;  // Check if the plugin is enabled
    // fn config(&self) -> Option<HashMap<String, String>>;  // Optional config for the plugin

    async fn start(&self, client: &AsyncClient);
}
