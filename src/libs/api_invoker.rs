use crate::libs::model::CovidData;
use actix_web::client::Client;
use color_eyre::Result;
use eyre::Report;

pub async fn get_covid_cases() -> Result<CovidData> {
    let response = Client::default()
        .get("https://covid19.th-stat.com/api/open/cases")
        .send()
        .await
        .map_err(|e| Report::msg(format!("{:?}", e)))?
        .json::<CovidData>()
        .limit(65535 * 128)
        .await?;
    Ok(response)
}
