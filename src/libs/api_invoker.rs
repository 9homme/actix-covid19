use crate::libs::model::CovidData;
use reqwest::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ApiInvoker: Send + Sync {
    async fn get_covid_cases(&self) -> Result<CovidData>;
}
pub struct ApiInvokerImpl();


impl ApiInvokerImpl {
    pub fn new() -> Self {
        ApiInvokerImpl()
    }
}

#[async_trait]
impl ApiInvoker for ApiInvokerImpl {
    async fn get_covid_cases(&self) -> Result<CovidData> {
        reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("https://covid19.th-stat.com/api/open/cases")
        .send()
        .await?
        .json::<CovidData>().await
    }
}


