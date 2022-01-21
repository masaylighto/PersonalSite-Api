use std::string;

/// this modual hold the responsiblity to deal with the database


use mysql::*;
use mysql::prelude::*;

use super::loger::log;

pub struct DbContext
{

     connection: PooledConn,
}
impl DbContext 
{
    /// will create a new instance from the DbContext and will return option
    pub fn new(server_ip:&str,username:&str,password:&str,db_name:&str)->Option<DbContext>
    {
        //this method used to open a connection to the database and it will return Option<PooledConn>
        //so we use match pattren to extract the value and there is none we will end the method and return None 
        //so the caller will not be apply to use this class if it falied to create a connection
        let connection= match DbContext::open_connection(server_ip,username,password,db_name)
        {
            Some(data) => data,
            None => {return None},
        };
        Some(DbContext {connection})
    }
    /// this method used to open a connection to the database and it will return Option<DbContext>
    fn open_connection(server_ip:&str,username:&str,password:&str,db_name:&str)->Option<PooledConn>
    {
        // create the object that contain the connection information
        let connection_information = DbContext::create_connection_information_object(server_ip,username,password,db_name);
        // pass the information to the pool::new the pool will return result struct that contain the pool object 
        let connection_pool=match Pool::new(connection_information)
        {
            Ok(data) => data,
            Err(err_msg) => 
            {
                    // log the connection error
                    log(&format!("Error on line {}  in db context : {}",line!(),err_msg));
                    // end the method excution if the connection failed to be opned 
                    return None;
            },
        };        
        // get the connection object from the pool and return it if it succesfuly created or return none of it the otherwise   
        match connection_pool.get_conn()
        {
            Ok(data) =>
            {
               Some(data)
            },
            Err(err_msg) => 
            {
                log(&format!("Error on line {}  in db context : {}",line!(),err_msg));
                return None;
            },
        }

      
      
    }

    /// this method check if user is exist 
    pub fn is_user_exist(mut self,username:&str,password:&str)->bool
    {
        //create a Paramiterized query this method will return option so we will use match pattren and if it did not successed then we end th method
       let statement=match self.connection.prep("SELECT COUNT(*) FROM ACCOUNTS WHERE USERNAME=:user_name AND PASSWORD=:pass_word")       
       {
            Ok(data) => data,
            Err(err_msg) => 
            {
                log(&format!("Error on line {} in db_context: {}",line!(),err_msg));
                return false;
            },
        };   
        //excute the paramitrized query with exec_first which will return one result as result<option<T>>
        let result=   self.connection.exec_first::<i32, Statement, Params>(statement,params!("user_name"=>username,"pass_word"=>password));
      // we used match pattren to extact the value if the value successfuly exctracted then store it in result var if it is not log the error and 
      // return false to the caller as indicator that checking user info didnt go well and to prevent the system from considering the information as right and login unauthorized user
        let result=  match result {
            Ok(data) => data,
            Err(err_msg) => 
            {
                log(&format!("Error on line {} in db context :{}",line!(),err_msg));
                return false;
            },
        };
        //  in the previous match we extracted from result in this match we will exctract from option
        let result = match result
        {
            Some(data) => data,
            None => {
                log(&format!("Error on line {} in db context : no data recived",line!()));
                return false;
            },
        };
        // as we did select count(*) in the query the returned value is the number of account with the enterd user name and password
        // so we check if the count is 1 then there is a user with this information and thus we register his login
        // if the result was zero then no user with this information
        //if the result is above zero 1 then there more than one user with this information and that might indecate a sql injection in the query
        // so we compare result to 1 and thus return bool as result to the caller
        result==1 

    }     
    /// this method used to create an object that contain the necessary information to connect to the database
    fn create_connection_information_object(server_ip:&str,username:&str,password:&str,db:&str)->OptsBuilder
    {
        OptsBuilder::new()
        .user(Some(username))
        .pass(Some(password))
        .ip_or_hostname(Some(server_ip))
        .db_name(Some(db))
    }
    

}

