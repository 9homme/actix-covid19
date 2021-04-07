use actix_covid19::libs::api_invoker::ApiInvoker;
use actix_covid19::libs::repository::Repository;
use actix_covid19::libs::{
    handler::*,
    model::{CovidCase, CovidData, CovidProvince, CovidSummary, NewUser, User},
};
use actix_web::{test, web, App};
use async_trait::async_trait;
use chrono::{Duration, Local};
use color_eyre::Result;
use mockall::*;
use mongodb::results::InsertOneResult;
use std::{ops::Add, sync::Arc};

#[actix_rt::test]
async fn test_health_get() {
    let mut app = test::init_service(App::new().route("/health", web::get().to(health))).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_covid19_get() {
    let date = Local::now().naive_local();

    mock! {
        DB {}
        #[async_trait]
        impl Repository for DB{
            async fn add_user(&self, new_user: NewUser) -> Result<InsertOneResult>;
            async fn get_user(&self, username: &String) -> Result<Option<User>>;
        }
    }

    mock! {
        API {}
        #[async_trait]
        impl ApiInvoker for API{
            async fn get_covid_cases(&self) -> reqwest::Result<CovidData>;
        }
    }

    let mut mock_api = MockAPI::new();
    mock_api.expect_get_covid_cases().returning(move || {
        Ok(CovidData {
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
        })
    });
    let mut mock_db = MockDB::new();
    mock_db.expect_get_user().returning(|_| Ok(Some(User{
        id: None,
        username: "orm".to_string(),
        password_hash: "f89a0c696a6553a9cedd5e32fe65bcfb17bf083c914dcc5a36e629248c469927a292e120bdd87a8cd79533ec1184958e45f0d2fcd33a70eb0b7fbaec719abfca".to_string()
    })));

    let arc_db: Arc<dyn Repository + Send + Sync> = Arc::new(mock_db);
    let arc_api: Arc<dyn ApiInvoker + Send + Sync> = Arc::new(mock_api);

    let mut app = test::init_service(
        App::new()
            .data(arc_db.clone())
            .data(arc_api.clone())
            .service(web::scope("/app").route("/covid19", web::get().to(covid19))),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/app/covid19")
        .header("Authorization", "Basic b3JtOm9ybTEyMw==")
        .to_request();

    let resp: CovidSummary = test::read_response_json(&mut app, req).await;

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

    assert_eq!(expected.data.len(), resp.data.len());
    for expected_province in expected.data {
        let result_province = resp
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
