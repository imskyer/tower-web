use response::{IntoResponse, Serializer};
use routing::{RouteMatch, RouteSet};
use util::BufStream;

use bytes::Buf;
use futures::Future;
use http;

/// A resource
///
/// TODO: Should `Clone + Send + 'static` be hard coded?
pub trait Resource: Clone {
    /// Identifies a route.
    type Destination: Clone + Send + Sync + 'static;

    /// Buffer yielded by the body. Represents a chunk of the body.
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: BufStream<Item = Self::Buf, Error = ::Error>;

    /*
    /// Responses returned by the resource
    type Response: IntoResponse<Buf = Self::Buf, Body = Self::Body>;
    */

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error>;

    /// Return the routes associated with the resource.
    fn routes(&self) -> RouteSet<Self::Destination>;

    fn dispatch<In: BufStream>(
        &mut self,
        destination: Self::Destination,
        route_match: RouteMatch,
        body: In,
    ) -> Self::Future;
}

/// Convert a value into a `Resource`
pub trait IntoResource<S> {
    type Resource: Resource;

    fn into_resource(self, serializer: S) -> Self::Resource;
}
