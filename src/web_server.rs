use actix_web::{App, HttpServer};
#[path = "web_server/storage/file_manger.rs"]
mod file_manger;
#[path = "web_server/parsers/form_parser.rs"]
mod form_parser;
#[path = "web_server/pages/page.rs"]
mod page;
#[path = "web_server/api_endpoints/image.rs"]
mod image;
#[path = "web_server/storage/loger.rs"]
mod loger;
#[path = "web_server/storage/database/db_context.rs"]
mod db_context;
/// Run the web server
#[actix_web::main]
pub async fn start_the_server()-> std::io::Result<()>
{
    let db= db_context::DbContext::new("127.0.0.1","root","","ali_miracle").unwrap();
    let r= db.get_users(Some(1),Some(0));
    print!("{:?}",r);
    // start new instance form the server and set the main there api endpoint method 
    HttpServer::new(|| {
        App::new()
            .service(image::save_image)// this service is responsible for storing  img giving by user 
            .service(image::dynamic_img_loader) //this method is responsible for giving the stored img to the user 
            .service(page::dynamic_page_loader)// this service is responsible for giving the html file to the client
            .service(page::dynamic_assets_loader) // this service is responsible for giving the js,cs file to the client
            .service(page::re_route_to_index) // re route the / into home/home.html as it is the first page in the website
    })
    // bind the server into the specifed ip address
    .bind("127.0.0.1:81")?
    .run()
    .await
}
