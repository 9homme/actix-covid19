use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CovidSummary {
    pub data: Vec<CovidProvince>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CovidProvince {
    pub province: String,
    pub count: i32,
    #[serde(with = "date_format")]
    pub last_date: NaiveDateTime,
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
    #[serde(with = "date_format")]
    pub confirm_date: NaiveDateTime,
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

mod date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
