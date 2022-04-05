use std::ops::Deref;

use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
};

use crate::authn::AuthenticatedUser;

// We have a new type for this to differentiate between the URL for this backend service and that
// for others
#[derive(Clone, Debug)]
pub(crate) struct PluginRegistryEndpointUrl(url::Url);

impl From<url::Url> for PluginRegistryEndpointUrl {
    fn from(u: url::Url) -> Self {
        Self(u)
    }
}

impl Deref for PluginRegistryEndpointUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(handler);
}

// TODO: use 'tail' do rebuild the path to backend (drop outer scope path)
#[post("/{tail:.*}")]
pub(crate) async fn handler(
    req: HttpRequest,
    payload: web::Payload,
    backend_url: web::Data<PluginRegistryEndpointUrl>,
    client: web::Data<awc::Client>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let url = backend_url.get_ref().deref().clone();

    super::fwd_request_to_backend_service(req, payload, url, client.get_ref().clone()).await
}
