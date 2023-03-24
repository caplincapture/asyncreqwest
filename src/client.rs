use reqwest::{Error, IntoUrl, Method, Request, RequestBuilder, Response};
use std::time::Duration;

pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let inner = reqwest::Client::builder()
            .user_agent("horo bot/1.0")
            .build()?;

        Ok(Self { inner })
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.inner.request(method, url)
    }

    pub async fn execute(&self, req: Request) -> Result<Response, Error> {
        let exec = || async {
            self.inner
                .execute(req.try_clone().unwrap())
                .await
                .and_then(|r| r.error_for_status())
                .map_err(backoff::Error::Transient)
        };

        let mut backoff = backoff::ExponentialBackoff::default();

        use backoff_futures::BackoffExt;

        let f = async {
            backoff::retry(backoff, exec);
            /* exec.with_backoff(&mut backoff).await.map_err(|e| match e {
                backoff::Error::Permanent(e) | backoff::Error::Transient{err: e,..} => e,
            }) */
        };

        fn check<F, O>(f: F) -> F
        where
            F: std::future::Future<Output = O> + Send,
        {
            f
        }
        check(f).await
    } 
}