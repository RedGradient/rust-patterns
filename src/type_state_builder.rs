use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Default, Clone)]
pub struct NoUrl;
#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct RequestBuilder<U> {
    url: U,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl RequestBuilder<Url> {
    pub fn build(self) -> Result<Request> {

        let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(
            Request {
                url: self.url.0,
                method,
                headers: self.headers,
                body: self.body,
            }
        )
    }
}

impl<U> RequestBuilder<U> {

    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method.insert(method.into());
        self
    }
    pub fn header(mut self, header: impl Into<(String, String)>) -> Self {
        self.headers.push(header.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.insert(body.into());
        self
    }
}
