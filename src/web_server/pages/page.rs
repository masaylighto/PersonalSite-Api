use actix_web::{HttpResponse, get};
#[get("/")]
pub async fn index() -> HttpResponse 
{ 
    // Get Page Content form file during completion and store it into the app using include_str
    let html = include_str!("../../front_end/home_page/home.html");
    // return the page content into the client
    HttpResponse::Ok().body(html)
}
