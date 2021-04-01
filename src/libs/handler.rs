use actix_web::{HttpResponse, Responder};
use itertools::Itertools;

use crate::libs::model::{CovidData, CovidProvince, CovidSummary};

pub async fn covid19() -> impl Responder {
    match super::api_invoker::get_covid_cases().await {
        Ok(data) => HttpResponse::Ok().json(covid19_summary(data)),
        Err(e) => HttpResponse::BadRequest().json(e),
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
        })
        .collect::<Vec<CovidProvince>>();
    CovidSummary {
        data: covid_provinces,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::libs::model::CovidCase;

    #[test]
    fn test_covid19_summary() {
        let input = CovidData {
            data: vec![
                CovidCase {
                    confirm_date: None,
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
                    confirm_date: None,
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
                    confirm_date: None,
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
                    confirm_date: None,
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
                },
                CovidProvince {
                    province: String::from("chiang mai"),
                    count: 1,
                },
                CovidProvince {
                    province: String::from("ไม่ระบุ"),
                    count: 1,
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
        }
    }
}
