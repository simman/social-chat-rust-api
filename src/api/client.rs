use crate::config::config::HttpConfig;
use crate::config::constant::{CHAT_SDK, SDK_CONFIG};
use nanoid::nanoid;
use reqwest::header::HeaderMap;
use reqwest::{Proxy};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};


use reqwest_tracing::TracingMiddleware;

static CONNECT_TIMEOUT: u64 = 15;

pub struct ApiClient {
    pub c: ClientWithMiddleware,
}

pub(crate) enum ClientTypeEnum {
    COMMON,
    IM,
}

impl ApiClient {
    pub(crate) fn new_client(client_type: ClientTypeEnum) -> ApiClient {
        let http_config: &HttpConfig;
        match client_type {
            ClientTypeEnum::COMMON => http_config = &SDK_CONFIG.http,
            ClientTypeEnum::IM => http_config = &SDK_CONFIG.im_server,
        }

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", http_config.user_agent.parse().unwrap());
        headers.insert("Sdk-Version", CHAT_SDK.version.parse().unwrap());
        headers.insert("request-id", nanoid!().parse().unwrap());
        headers.insert("terminalType", "2".parse().unwrap());

        let mut client_builder = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(CONNECT_TIMEOUT))
            .connection_verbose(true)
            .cookie_store(true)
            .gzip(true)
            .default_headers(headers);

        if let Some(_proxy) = &http_config.proxy {
            client_builder = client_builder.proxy(Proxy::all(_proxy).unwrap());
        }

        let client = client_builder.build().unwrap();

        // let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);

        ApiClient {
            c: ClientBuilder::new(client)
                .with(TracingMiddleware::default())
                // .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build(),
        }
    }

    pub fn config(&self) {
        // self.client.post("").headers()
    }
}
