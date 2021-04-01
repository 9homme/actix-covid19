use crate::libs::model::{CovidData, Error};
use actix_web::client::Client;

pub async fn get_covid_cases() -> Result<CovidData, Error> {
    let response = Client::default()
        .get("https://covid19.th-stat.com/api/open/cases")
        .send()
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error {
                message: format!("{:?}", e),
            }
        })?
        .json::<CovidData>()
        .limit(65535 * 128)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error {
                message: format!("{:?}", e),
            }
        });
    response
}
