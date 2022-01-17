use actix_web::{App, HttpServer};

#[path = "web_server/helper_mod/file_manger.rs"]
mod file_manger;
#[path = "web_server/parsers/form_parser.rs"]
mod form_parser;
#[path = "web_server/pages/page.rs"]
mod page;
#[path = "web_server/api_endpoints/image.rs"]
mod image;
/// Create an Index Page


/// Run the web server
#[actix_web::main]
pub async fn start_the_server()-> std::io::Result<()>
{
    // start new instance form the server and set the main there api endpoint method 
    HttpServer::new(|| {
        App::new()
            .service(image::get_image_as_byte_array)
            .service(image::save_image)
            .service(page::index)
    })
    // bind the server into the specifed ip address
    .bind("127.0.0.1:8083")?
    .run()
    .await

}
