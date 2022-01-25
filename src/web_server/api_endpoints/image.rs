use actix_multipart::{Multipart};
use actix_web::{ HttpResponse, get, Responder, post, web};
use futures_util::TryStreamExt;
use crate::web_server::{file_manger, form_parser};
use super::logger::log;
/// this method return  buffer of bytes that represent the requested img to the client
#[get("{folder}/images/{file}")]
// web::Path<(String,String)> parameter will parse the http request of type get 
async fn dynamic_img_loader(url:web::Path<(String,String)>) -> impl Responder 
{
    let url=url.into_inner();
    // we use the method file_manger::read_file which read the data of img you passed it path 
    // the return type of the method is Result<Vec<u8>,String> 
    // string is returned when there is error and it indicate the error and we return it to the client
    // the vec<u8> represent the img bytes and we return it to the client
   match file_manger::read_file(&format!("front_end/{}/images/{}",url.0,url.1)).await
   {
        Ok(data)=>
        {
            return HttpResponse::Ok().body(data);
        },
        Err(err_msg) => 
        {
            log(&format!("Error on Line {} : {}",line!(),err_msg));
            return HttpResponse::Ok().body(err_msg);
        },
   }
}
/// Save Image that Posted by client with  method="post" enctype="multipart/form-data" 
/// the post should contain three field the first one is the directory you want to store the img into it
/// second the img name third the img bytes 
#[post("/SaveImage")]
// the multipart parameter represent the form buffer and you can get the data from it one by ine throw the method try_next
async fn save_image(mut payload: Multipart) -> impl Responder 
{  
    // here we use the form_parser::get_string method which is responsible to get a string out of the post field we pass into it the first field using try next method 
    // if you are wandering why we use await method ,that is because all the method in this app are async method
    // the get string return Option<String> and that why we used Match pattern on it to extract the data
    let folder_name:String = match form_parser::get_string(payload.try_next().await).await 
    {
        Some(data)=> data,
        None=> return HttpResponse::Ok().body("Fail to Parse First Field in the Post")
    };
    // we apply the same method we mention above again but this time for the next field of received post which represent the file name 
    let file_name:String = match form_parser::get_string(payload.try_next().await).await 
    {
        Some(data)=> data,
        None=> return HttpResponse::Ok().body("Fail to Parse Second Field in the Post")
    };  
    // here we apply the method form_parser::get_bytes to get the img buffer form the third post 
    // the return type of this method is Option<Vec<u8>> and that why we used the match pattern to extract the data    
    let img_buffer:Vec<u8> = match  form_parser::get_bytes(payload.try_next().await).await
    {
        // if the data is present in the option we assign it to the img_buffer 
        Some(data)=> data,
        // if there is no data present we end the execution and return failed message to the used
        None=> return HttpResponse::Ok().body("Fail to Parse third Field in the Post")
    }; 
    // here we used the file_manger::create_file to create file with the information we extracted above
    // the method return true if it succeed and false other wise
    // we check the return value if its true we return Done to the used if it not we return Fail
    if file_manger::create_file(file_name,folder_name,img_buffer).await
    {
        return HttpResponse::Ok().body("Done");
    }
    return HttpResponse::Ok().body("Fail");
}