use headers::HeaderMapExt;
use reqwest::{header::HeaderMap, Request, RequestBuilder};

pub trait TypedHeaderExt: private::Sealed + Sized {
    fn typed_header<H>(self, header: H) -> Self
    where
        H: headers::Header;
}

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

impl TypedHeaderExt for Request {
    fn typed_header<H>(mut self, header: H) -> Self
    where
        H: headers::Header,
    {
        self.headers_mut().typed_insert(header);
        self
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for RequestBuilder {}
    impl Sealed for Request {}
}
