/// this file used to deal with operation on files
use std::{fs::{self, File}, io::{Write, Read}};
use actix_web::web;
/// this method create a file it receive the directory where the file will be stored and the file name and its data
pub async fn create_file(file_name:String,folder_name:String,buffer:Vec<u8>)->bool
{   // first here we use fs::create_dir_all method which create the directory if its not exist and if it did field it will return false 
    if fs::create_dir_all(&folder_name).is_err()
    {
        return false;
    } 
    // we combine the folder name and the file to create the path of file
    // and that after we created the directory with the method above 
    // so now we need both file name and directory name so the method std::fs::File::create will now were to create the file
    let full_path:String=format!("{}/{}", folder_name,file_name);   
    // ue use teh web:block which block the execution of the parent method till file::create finish its job 
    //we apply match pattern to extract the the data returned which represent the file handle which will be used to fill the file with the data in content parameter
    let mut file:File = match web::block(move || std::fs::File::create(&full_path)).await
    {
        Ok(data) => data,
        Err(_) => 
        {
            return false;
        }
    };  
    // we use the handle returned above to write  buffer into the file
    file.write_all(&buffer).map(|_| file).is_ok()
}
/// this method used to read file buffer and return it to the user
pub async fn read_file(file_path:&String) ->Result<Vec<u8>,String>
{
    // we open handle to the file
    let file = File::open(&file_path);
    //check if the handle is invalid if it is end the execution and return string represent the error into the caller 
    if file.is_err() 
    {
        return Err(format!("File not found {}", &file_path));
    }
    // get the file metadata which contain the file buffer length 
    // the length will be used to determine the size of the array that will contain it
    let metadata = fs::metadata(&file_path);    
    // check if reading meta data failed if its end the method execution and return a string that represent the error
    if metadata.is_err() 
    {
        return Err(String::from("couldn't read File metadata"));
    }
    // create the array that will contain the file data
    let mut buffer: Vec<u8> = vec![0; metadata.unwrap().len() as usize];
    //use the file handle that created above to read the buffer 
    //and check if its field if it did field the is_err() will return true and the if condition code will execute and return string that represent the error
    if file.unwrap().read(&mut buffer).is_err() 
    {
        return Err(String::from("couldn't read File Data"));
    }
    // of it successes return the buffer
    Ok(buffer)
}
pub async fn read_file_as_string(path:&String)->Option<String>
{
    //we read file data from the file into vec<u8> then we use match to extract it from result struct 
    let content = match  read_file(&path).await
    {
        Ok(buffer)=>buffer,
        Err(_)=>
        {
        return None;
        }
    }; 
    // convert it from vec<u8> into Cow<string> cause we are returning a css or js or any other type of resource
    let content =String::from_utf8_lossy(&content);
    //convert Cow<String> into String
    let content =format!("{}",content);
    Some(content)
}