use actix_web::{error, web, HttpResponse, Responder};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Debug, Serialize, Deserialize)]
pub struct CalcForm {
    pub value1: Option<String>,
    pub value2: Option<String>,
    pub opt: Option<String>,
}

impl CalcForm {
    pub fn calc(&self) -> anyhow::Result<i32> {
        let value1 = self.value1.clone().unwrap().parse::<i32>()?;
        let value2 = self.value2.clone().unwrap().parse::<i32>()?;
        let opt = self.opt.clone().unwrap().parse::<i32>()?;

        let result = match opt {
            1 => value1 + value2,
            2 => value1 - value2,
            3 => value1 * value2,
            4 => value1 / value2,
            5 => value1 % value2,
            _ => return Err(Error::msg("Please input 1 to 5")),
        };
        Ok(result)
    }
}

pub async fn get_calc(tera: web::Data<Tera>) -> impl Responder {
    let body = tera
        .render("pages/index.html", &tera::Context::new())
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))
        .unwrap();
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(body)
}

pub async fn post_calc(form: web::Form<CalcForm>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    let form = form.into_inner();
    match form.calc() {
        Ok(result) => context.insert("result", &result),
        Err(err) => context.insert("error", &err.to_string()),
    }

    let body = tera
        .render("pages/result.html", &context)
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))
        .unwrap();
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::dev::ServiceResponse;
    use actix_web::{http::StatusCode, test, App};

    async fn test_service() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    > {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        test::init_service(
            App::new().app_data(web::Data::new(tera.clone())).service(
                web::resource("/tera")
                    .route(web::get().to(get_calc))
                    .route(web::post().to(post_calc)),
            ),
        )
        .await
    }

    #[actix_web::test]
    async fn get() {
        let test_service = test_service().await;
        let req = test::TestRequest::get().uri("/tera").to_request();
        let resp = test::call_service(&test_service, req).await;

        println!("{:?}", resp.headers());
        println!("{:?}", resp.response().body());
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn post() {
        let test_service = test_service().await;
        let form = CalcForm {
            value1: Some("100".to_string()),
            value2: Some("200".to_string()),
            opt: Some("1".to_string()),
        };
        let req = test::TestRequest::post()
            .uri("/tera")
            .set_form(&form)
            .to_request();
        let resp = test::call_service(&test_service, req).await;

        println!("{:?}", resp.headers());
        println!("{:?}", resp.response().body());
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
