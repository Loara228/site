use actix_web::{get, web, HttpResponse, Responder};

use crate::SiteData;


#[get("/{user_id}")]
async fn user(data: web::Data<SiteData>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("id", &user_id);
    ctx.insert("username", "example username");
    let html = data.tera.render("users/user.html", &ctx).unwrap();
    web::Html::new(html)
}