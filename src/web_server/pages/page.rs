/// this page represent the route for static html file
use actix_web::{HttpResponse,http::{StatusCode}, get, web};
use super::{file_manger, logger::log};
///this method get the static html content though the url provided by client during the http request and return it to the user 
#[get("/{folder}/{file}")]
// first the parameter path:web::Path<(String,String)> mean that path gonna have two value of string the first one is the folder and the other one is the file name
pub async fn dynamic_page_loader(url:web::Path<(String,String)>) -> HttpResponse 
{   // convert from Path<tuple> int tuple
    let url =url.into_inner();     
    //we format it into path for the requested file
    let path= format!("front_end/{}/{}",url.0,url.1);
    // Get Page Content form file and return it back to client
    get_static_file_content(path).await
}
///this method used to get assets like css js etc through the url provided by client during the http request
#[get("/assets/{folder}/{file}")]
// first the parameter path:web::Path<(String,String)> mean that path gonna have two value of string the first one is the folder and the other one is the file name
pub async fn dynamic_assets_loader(url:web::Path<(String,String)>) -> HttpResponse 
{  
    //we extract the tuple from path
    let  url= url.into_inner();  
    //we format it into path for the requested file
    let path= format!("front_end/assets/{}/{}",url.0,url.1);
    // Get Page Content form file and return it back to client
    get_static_file_content(path).await
}
///this method used to get a static file content of the text type as HttpResponse
pub async fn get_static_file_content(path:String) -> HttpResponse 
{
    //reading the file data as string this method return option<string> so we used match
    let content:String= match  file_manger::read_file_as_string(&path).await
    {
        Some(data)=>data,
        None=>
        {   // if failed to read the data ,Log it
            log(&format!("Error on line {} : the file in the path {} couldn't be read",line!(),&path));       
            return HttpResponse::new(StatusCode::NOT_FOUND);
        }
    }; 
     // return the  content into caller method
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(content)
}
/// this method convert url from / to /home/home.html
#[get("/")]
pub async fn re_route_to_index() -> HttpResponse 
{
    HttpResponse::Found().header("Location", "/home/home.html").finish() 
}