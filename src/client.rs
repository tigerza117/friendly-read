use reqwest::{Client, RequestBuilder};

static USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.61 Safari/537.36 RuxitSynthetic/1.0 v8007188160821992392 t5908864234688580170 athf552e454 altpriv cvcv=2 smf=0";

pub fn builder(method: reqwest::Method, link: &str) -> RequestBuilder {
    return Client::builder()
        .build()
        .ok()
        .expect("Fail to create client")
        .request(method, link)
        .header(reqwest::header::USER_AGENT, USER_AGENT);
}
