use codegen::CallSite;
use extract::Context;
use routing::Params;

use http::Request;

// TODO: Does this live here?
#[derive(Debug)]
pub struct RouteMatch {
    /// The matched HTTP request head
    request: Request<()>,

    /// Extracted route parameters
    params: Params,
}

impl RouteMatch {
    /// Create a new `RouteMatch`
    pub(crate) fn new(request: Request<()>, params: Params) -> Self {
        RouteMatch {
            request,
            params,
        }
    }

    pub fn extract_context<'a>(&'a self, call_site: &'a CallSite) -> Context<'a> {
        Context::new(call_site, self)
    }

    pub(crate) fn request(&self) -> &Request<()> {
        &self.request
    }

    pub(crate) fn params(&self) -> &Params {
        &self.params
    }
}
