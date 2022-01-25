///this module have the method that deal with Accounts Table

use super::{db_context::DbContext, logger::log,super::web_server::db_module};
use mysql::*;
use mysql::prelude::*;

impl DbContext {
    /// this method check if user is exist 
    pub fn is_user_exist(self,username:&str,password:&str)->bool
    {
        // get the count of user that match the information provided by the user
        let  result= self.get_cell::<i32>("SELECT COUNT(*) FROM ACCOUNTS WHERE USERNAME=:user_name AND PASSWORD=:pass_word",params!("user_name"=>username,"pass_word"=>password));       
        // return false to the caller as indicator that checking user info didn't go well and to prevent the system from considering the information as right and login unauthorized user
        let result = match result
        {
            Some(data) => data,
            None =>
            {
                log(&format!("Error on line {} in db context : no data received",line!()));
                return false;
            },
        };
        // as we did select count(*) in the query the returned value is the number of account with the entered user name and password
        // so we check if the count is 1 then there is a user with this information and thus we register his login
        // if the result was zero then no user with this information
        //if the result is above zero 1 then there more than one user with this information and that might indicate a sql injection in the query
        // so we compare result to 1 and thus return bool as result to the caller
        result==1 
    }     
    /// this method will create a new user in the database
    pub fn create_user(self,name:&str,username:&str,email:&str,password:&str,is_owner:&str)->bool
    {
       // insert the new user information into the database
       let  is_succeed= self.execute_parameterized("INSERT INTO ACCOUNTS (NAME,USERNAME,EMAIL,PASSWORD,ISOWNER) VALUES(:name,:user_name,:email,:pass_word,:is_owner)",params!("name"=>name,"user_name"=>username,"email"=>email,"pass_word"=>password,"is_owner"=>is_owner));       
       // return the result
       is_succeed    
    }
    /// this method will update the user specify by the id in the database
    pub fn update_user(self,id:&str,name:&str,username:&str,email:&str,password:&str,is_owner:&str)->bool
    {
        // update the user information in the database
        let  is_succeed= self.execute_parameterized("UPDATE ACCOUNTS SET NAME=:name,USERNAME=:user_name,EMAIL=:email,PASSWORD=:pass_word,ISOWNER=:is_owner WHERE ID=:id",params!("name"=>name,"user_name"=>username,"email"=>email,"pass_word"=>password,"is_owner"=>is_owner,"id"=>id));       
        // return the result
        is_succeed    
    }    
     /// this method will delete the user specify by the id from the database ,if the user wasn't an admin
     pub fn delete_user(self,id:&str)->bool
     {
         // update the user information in the database
         let  is_succeed= self.execute_parameterized("DELETE FROM ACCOUNTS WHERE ID=:id and ISOWNER=0",params!("id"=>id));       
         // return the result
         is_succeed    
     }   
     /// this method will get all users or specific count of user as Vec of Struct User in db_module
     /// if the limit parameter is None then the method will get all user
     /// and if it is not it will extract it value and specify the amount of row it will select
    pub fn get_users(mut self,limit:Option<i32>,offset:Option<i32>)->Result<Vec<db_module::USER>>
    {
        // this is a mapping function used to map the result of a select query into a User struct
        let map_function=   |(id,name,username,email,is_owner)|
        {
            db_module::USER
            { 
            id: id, name: name, username: username, email: email, password:None, is_owner: is_owner 
            }
        };
        // we will preform a select query and map the result to a struct name User in db_module 
        // the function result will be  result struct that contain a vec of the type User which will contain all the users information
        // if there is no value in count then we just select all row
        if limit.is_none()
        {
          return  self.connection.query_map("SELECT  ID,NAME,USERNAME,EMAIL,ISOWNER FROM ACCOUNTS ",map_function)  ;
        }
        // force a value to number_of_row_to_skip by extracting its value from option and set zero as value if there is none
        let offset=match offset
        {
            Some(data) => data,
            None => 0,
        };
        //extract rows_count value
        let limit=limit.unwrap();
        self.connection.exec_map("SELECT  ID,NAME,USERNAME,EMAIL,ISOWNER FROM ACCOUNTS  limit :count offset :skip  ",params!("skip"=>offset,"count"=>limit),map_function)  

    } 
    ///get user by id, this method will return vec that contain the user info in its first index
    pub fn get_user_by_id(mut self,id:i32)->Result<Vec<db_module::USER>>
    {
       // this is a mapping function used to map the result of a select query into a User struct
        let map_function=   |(id,name,username,email,is_owner)|
        {
            db_module::USER
            { 
            id: id, name: name, username: username, email: email, password:None, is_owner: is_owner 
            }
        };
        // we will preform a select query and map the result to a struct name User in db_module 
        // the function result will be  result struct that contain a vec of the type User that will have the user info as user struct at it first index
        self.connection.exec_map("SELECT  ID,NAME,USERNAME,EMAIL,ISOWNER FROM ACCOUNTS where id=:id ",params!("id"=>id),map_function)    
    } 
}