use super::api_invoker::ApiInvoker;
use super::repository::Repository;
use crate::libs::model::{CovidData, CovidProvince, CovidSummary, NewUser};
use actix_web::{web, HttpResponse, Responder};
use blake2::{Blake2b, Digest};
use itertools::Itertools;
use std::sync::Arc;
use tracing::*;

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub async fn add_user(
    repository: web::Data<Arc<dyn Repository + Send + Sync>>,
    new_user: web::Json<NewUser>,
    _auth: super::auth::AuthenticatedUser,
) -> impl Responder {
    let result = repository.add_user(new_user.0).await;
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest().body(format!("{:?}", e)),
    }
}

pub async fn hash(web::Path(value): web::Path<String>) -> impl Responder {
    let hash = Blake2b::digest(value.as_ref());
    HttpResponse::Ok().body(format!("{:x}", hash))
}

pub async fn covid19(
    api_invoker: web::Data<Arc<dyn ApiInvoker + Send + Sync>>,
    auth: super::auth::AuthenticatedUser,
) -> impl Responder {
    debug!("Basic Authentication : {}", auth.user.username);
    match api_invoker.get_covid_cases().await {
        Ok(data) => HttpResponse::Ok().json(covid19_summary(data)),
        Err(e) => HttpResponse::BadRequest().body(format!("{:?}", e)),
    }
}

fn covid19_summary(covid_data: CovidData) -> CovidSummary {
    let covid_provinces = covid_data
        .data
        .into_iter()
        .into_group_map_by(|c| c.province.clone().unwrap_or(String::from("ไม่ระบุ")))
        .iter()
        .map(|(k, v)| CovidProvince {
            province: String::from(k),
            count: v.len() as i32,
            last_date: v
                .iter()
                .map(|c| c.confirm_date)
                .max()
                .expect("Get max date from cases"),
        })
        .sorted_by(|a, b| Ord::cmp(&b.count, &a.count))
        .collect::<Vec<CovidProvince>>();
    CovidSummary {
        data: covid_provinces,
    }
}

#[cfg(test)]
mod test {
    use std::ops::Add;

    use chrono::{Duration, Local};

    use crate::libs::model::CovidCase;

    use super::*;

    #[test]
    fn test_covid19_summary() {
        let date = Local::now().naive_local();
        let input = CovidData {
            data: vec![
                CovidCase {
                    confirm_date: date,
                    no: None,
                    age: None,
                    gender: None,
                    gender_en: None,
                    nation: None,
                    nation_en: None,
                    province: Some(String::from("bangkok")),
                    province_id: None,
                    district: None,
                    province_en: None,
                    detail: None,
                    stat_quarantine: None,
                },
                CovidCase {
                    confirm_date: date,
                    no: None,
                    age: None,
                    gender: None,
                    gender_en: None,
                    nation: None,
                    nation_en: None,
                    province: Some(String::from("chiang mai")),
                    province_id: None,
                    district: None,
                    province_en: None,
                    detail: None,
                    stat_quarantine: None,
                },
                CovidCase {
                    confirm_date: date.add(Duration::days(1)),
                    no: None,
                    age: None,
                    gender: None,
                    gender_en: None,
                    nation: None,
                    nation_en: None,
                    province: Some(String::from("bangkok")),
                    province_id: None,
                    district: None,
                    province_en: None,
                    detail: None,
                    stat_quarantine: None,
                },
                CovidCase {
                    confirm_date: date,
                    no: None,
                    age: None,
                    gender: None,
                    gender_en: None,
                    nation: None,
                    nation_en: None,
                    province: None,
                    province_id: None,
                    district: None,
                    province_en: None,
                    detail: None,
                    stat_quarantine: None,
                },
            ],
        };

        let expected = CovidSummary {
            data: vec![
                CovidProvince {
                    province: String::from("bangkok"),
                    count: 2,
                    last_date: date.add(Duration::days(1)),
                },
                CovidProvince {
                    province: String::from("chiang mai"),
                    count: 1,
                    last_date: date,
                },
                CovidProvince {
                    province: String::from("ไม่ระบุ"),
                    count: 1,
                    last_date: date,
                },
            ],
        };

        let result = covid19_summary(input);
        assert_eq!(expected.data.len(), result.data.len());
        for expected_province in expected.data {
            let result_province = result
                .data
                .iter()
                .find(|p| p.province == expected_province.province);
            assert_eq!(result_province.is_some(), true);
            assert_eq!(expected_province.count, result_province.unwrap().count);
            assert_eq!(
                expected_province.last_date.date(),
                result_province.unwrap().last_date.date()
            )
        }
    }
}
