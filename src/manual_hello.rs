use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/hey", web::get().to(manual_hello));
}

async fn manual_hello(thread_index: web::Data<u16>) -> HttpResponse {
    HttpResponse::Ok()
        .header("thread-id", thread_index.to_string())
        .body("Hey there!")
}

#[cfg(test)]
mod test {
    use crate::manual_hello::manual_hello;
    use actix_web::web;

    #[actix_rt::test]
    async fn manual_hello_works() {
        let res = manual_hello(web::Data::new(5u16)).await;
        assert!(res.status().is_success());
    }
}
