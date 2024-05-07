use goauth::scopes::Scope;
use http::{uri::InvalidUri, Uri};
use serde_with::serde_as;
use snafu::{ResultExt, Snafu};

use vector_lib::{
    codecs::decoding::{DeserializerConfig, FramingConfig},
    config::{LogNamespace, SourceOutput},
    configurable::configurable_component,
};

use crate::{
    config::{SourceConfig, SourceContext},
    gcp::{GcpAuthConfig, PUBSUB_URL},
    serde::{default_decoding, default_framing_message_based},
};

#[derive(Debug, Snafu)]
pub(crate) enum GcpGcsError {
    #[snafu(display("Invalid endpoint URI: {}", source))]
    Uri { source: InvalidUri },
}

/// Configuration of the `gcp_gcs` source.
#[serde_as]
#[configurable_component(source(
    "gcp_gcs",
    "Fetch observability events from GCP's Cloud Storage."
))]
#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct GcpGcsConfig {
    /// The project name.
    #[configurable(metadata(docs::examples = "my-log-source-project"))]
    pub project: String,

    /// The subscription within the project.
    #[configurable(metadata(docs::examples = "my-vector-source-subscription"))]
    pub subscription: String,

    /// The endpoint from which to pull data.
    #[configurable(metadata(docs::examples = "https://us-central1-pubsub.googleapis.com"))]
    #[serde(default = "default_endpoint")]
    pub endpoint: String,

    #[configurable(derived)]
    #[serde(default = "default_framing_message_based")]
    #[derivative(Default(value = "default_framing_message_based()"))]
    pub framing: FramingConfig,

    #[configurable(derived)]
    #[serde(default = "default_decoding")]
    #[derivative(Default(value = "default_decoding()"))]
    pub decoding: DeserializerConfig,

    #[serde(flatten)]
    pub auth: GcpAuthConfig,
}

fn default_endpoint() -> String {
    PUBSUB_URL.to_string()
}

#[async_trait::async_trait]
#[typetag::serde(name = "gcp_gcs")]
impl SourceConfig for GcpGcsConfig {
    async fn build(&self, _ctx: SourceContext) -> crate::Result<crate::sources::Source> {
        let auth = self.auth.build(Scope::PubSub).await?;
        let mut endpoint: Uri = self.endpoint.parse().context(UriSnafu)?;
        auth.apply_uri(&mut endpoint);
        let _token_generator = auth.spawn_regenerate_token();

        todo!()
    }

    fn outputs(&self, _global_log_namespace: LogNamespace) -> Vec<SourceOutput> {
        todo!()
    }

    fn can_acknowledge(&self) -> bool {
        true
    }
}

impl_generate_config_from_default!(GcpGcsConfig);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<GcpGcsConfig>();
    }
}
