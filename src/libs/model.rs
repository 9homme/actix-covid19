use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CovidSummary {
    pub data: Vec<CovidProvince>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CovidProvince {
    pub province: String,
    pub count: i32,
}

/*
{"Data":[
{"ConfirmDate":"2021-01-14 00:00:00",
"No":"11262","Age":null,"Gender":"\u0e44\u0e21\u0e48\u0e23\u0e30\u0e1a\u0e38\u0e40\u0e1e\u0e28",
"GenderEn":"Unknown","Nation":"\u0e44\u0e21\u0e48\u0e17\u0e23\u0e32\u0e1a","NationEn":"Unknown",
"Province":"\u0e2a\u0e21\u0e38\u0e17\u0e23\u0e2a\u0e32\u0e04\u0e23","ProvinceId":62,"District":"",
"ProvinceEn":"Samut Sakhon","Detail":null,"StatQuarantine":0}]}
 */
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "PascalCase"))]
pub struct CovidData {
    pub data: Vec<CovidCase>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "PascalCase"))]
pub struct CovidCase {
    pub confirm_date: Option<String>,
    pub no: Option<String>,
    pub age: Option<f32>,
    pub gender: Option<String>,
    pub gender_en: Option<String>,
    pub nation: Option<String>,
    pub nation_en: Option<String>,
    pub province: Option<String>,
    pub province_id: Option<i32>,
    pub district: Option<String>,
    pub province_en: Option<String>,
    pub detail: Option<String>,
    pub stat_quarantine: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "PascalCase"))]
pub struct Error {
    pub message: String,
}
