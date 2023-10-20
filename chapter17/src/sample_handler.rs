use actix_web::{web, HttpResponse, Responder};
use log::info;
use mime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCalc {
    pub value1: Option<String>,
    pub value2: Option<String>,
    pub answer: Option<String>,
}

impl AddCalc {
    pub fn calc(&mut self) {
        let f = |v: &String| {
            if v.eq("") {
                return 0;
            } else {
                v.parse::<i32>().unwrap()
            }
        };

        let value1 = self.value1.as_ref().map_or_else(|| 0, |v| f(v));
        let value2 = self.value2.as_ref().map_or_else(|| 0, |v| f(v));
        self.value1 = Some(value1.to_string());
        self.value2 = Some(value2.to_string());
        self.answer = Some((value1 + value2).to_string());
    }
}

impl ToString for AddCalc {
    fn to_string(&self) -> String {
        format!(
            "{} + {} = {}",
            self.value1.as_ref().unwrap(),
            self.value2.as_ref().unwrap(),
            self.answer.as_ref().unwrap()
        )
    }
}

pub async fn calc_1(value: web::Path<(String, String)>) -> impl Responder {
    let val1 = value.0.parse::<i32>().unwrap();
    let val2 = value.1.parse::<i32>().unwrap();

    let result = format!("{} + {} = {}", val1, val2, val1 + val2);
    HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN_UTF_8)
        .body(result)
}

pub async fn calc_2(value: web::Query<AddCalc>) -> impl Responder {
    let mut val = value.into_inner();
    val.calc();
    info!("{:?}", val.to_string());
    HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN_UTF_8)
        .body(val.to_string())
}

pub async fn calc_3(value: web::Form<AddCalc>) -> impl Responder {
    let mut val = value.into_inner();
    val.calc();
    info!("{:?}", val.to_string());
    HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN_UTF_8)
        .body(val.to_string())
}

pub async fn calc_4(value: web::Json<AddCalc>) -> impl Responder {
    let mut val = value.into_inner();
    val.calc();
    info!("{:?}", val.to_string());
    HttpResponse::Ok()
        .content_type(mime::TEXT_PLAIN_UTF_8)
        .body(val.to_string())
}

pub async fn calc_5(value: web::Json<AddCalc>) -> impl Responder {
    let mut val = value.into_inner();
    val.calc();
    info!("{:?}", val.to_string());
    HttpResponse::Ok()
        .content_type(mime::APPLICATION_JSON)
        .json(val)
}
