use headers::HeaderMapExt;
use reqwest::{header::HeaderMap, Request, RequestBuilder};
use sealed::sealed;

#[sealed]
pub trait TypedHeaderExt: Sized {
    fn typed_header<H>(self, header: H) -> Self
    where
        H: headers::Header;
}

#[sealed]
impl TypedHeaderExt for RequestBuilder {
    fn typed_header<H>(self, header: H) -> Self
    where
        H: headers::Header,
    {
        let mut headers = HeaderMap::new();
        headers.typed_insert(header);
        self.headers(headers)
    }
}

#[sealed]
impl TypedHeaderExt for Request {
    fn typed_header<H>(mut self, header: H) -> Self
    where
        H: headers::Header,
    {
        self.headers_mut().typed_insert(header);
        self
    }
}
