/// this file used to parse post request of multipart/form-data
use actix_multipart::{Field, MultipartError, };
use futures_util::TryStreamExt;
use super::logger:: log;
/// this method take only one parameter which can be get bu using trynext on variable of  multipart which received from the post request ,
/// return string that represent tha data
pub async fn get_string(payload:Result<Option<Field>,MultipartError>)->Option<String>
{
    // get the bytes out of the payload parameter using get_bytes 
    let string_bytes=match  get_bytes(payload).await 
    {
        Some(data)=> data,
        None=>{return None;}
    };   
    //convert bytes into string and return it into the used 
    let data = format!("{}", String::from_utf8_lossy(&string_bytes));
    Some(data)
}
/// this method take only one parameter which can be get bu using trynext on variable of  multipart which received from the post request ,
/// return the payload data 
pub async fn get_bytes(field:Result<Option<Field>,MultipartError>)->Option<Vec<u8>>
{
    // here we extract the value field out of the parameter
    let field = match field
    {
        Ok(data) =>data,        
        Err(_) => 
        {
            log(&format!("Fail to Parse Payload First Result Field in the form {}",line!() ));
            return None;
        }
    };
    //we call get_field_bytes to get the bytes
    get_field_bytes(field).await  
}
/// a sub method that belong to get_bytes which responsible to get that post bytes
async fn get_field_bytes(field:Option<Field>)->Option<Vec<u8>>
{
    // we extract the field out of Option<field>
    let mut field = match field
    {
        Some(data) => data,
        
        None =>
        {
            log(&format!("Fail to Parse Payload  First Option Field in the form {}",line!() ));
            return None;
        }
    };
    //creating empty vector that will receive the data
    let mut bytes_vec=Vec::<u8>::new();
    // use iteration_count to check if there any buffer had been read 
    let mut iteration_count=0;
    // try_next on variable of type field will convert it into bytes and the return type is of Option<Bytes>
    // we use loop cause if the buffer is large the try next needed to be called more than one time
    while let Ok(data) = field.try_next().await 
    {
        // we extract the bytes
        let field=match data  
        {
            Some(data) =>data,
            None => 
            { 
                // if there is an iteration that happen before then break the loop abd go to returning the value
                // of no iteration did happen then that's mean that no value had been read out of Field parameter so  return None as Result
                if iteration_count>0 
                {                    
                    break
                }      
                log(&format!("Fail to Parse Payload First Result Field in the form {}",line!() ));   
                return  None;   
            }
        };
        // increase the iteration_count every loop
        iteration_count+=1;
        // append the bytes into the vec so we will have by the end vec that contain all the bytes
        bytes_vec.append(&mut field.to_vec());
    }
// return the vector
   Some(bytes_vec)
}