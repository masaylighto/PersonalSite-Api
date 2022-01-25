/// this module hold the responsibility to deal with the database
use mysql::*;
use mysql::prelude::*;

use super::logger::log;
mod db_module;

pub struct DbContext
{
   pub  connection: PooledConn,
}
impl DbContext 
{
    /// will create a new instance from the DbContext and will return option
    pub fn new(server_ip:&str,username:&str,password:&str,db_name:&str)->Option<DbContext>
    {
        //this method used to open a connection to the database and it will return Option<PooledConn>
        //so we use match pattern to extract the value and there is none we will end the method and return None 
        //so the caller will not be apply to use this class if it failed to create a connection
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
                    // end the method execution if the connection failed to be opened 
                    return None;
            },
        };        
        // get the connection object from the pool and return it if it successfully created or return none of it the otherwise   
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
       /// used to execute sql query with parameters
       pub fn execute_parameterized(mut self,query:&str,params:Params)->bool
       {
           //create a Parameterized query this method will return option so we will use match pattern and if it did not successes then we end th method
           let statement=match self.connection.prep(query)       
           {
               Ok(data) => data,
               Err(err_msg) => 
               {
                   log(&format!("Error on line {} in db_context: {}",line!(),err_msg));
                   return false;
               },
           };   
           //execute the parametrized query with exec_drop which will return result struct with no data 
           // we will use result to determine if the execution was completed successfully
           let result=   self.connection.exec_drop(statement,params);
           //the result come as Result  so we will use method Is_ok as return cause if the query executed successfully the value will be true
           // and that what we will return and if the query failed to execute then it will return false and that what we will return
           result.is_ok()
       }
    /// used to execute sql query and get one cell of data
    pub fn get_cell<T:FromRow>(mut self,query:&str,params:Params)->Option<T>
    {
        //create a Parameterized query this method will return option so we will use match pattern and if it did not successes then we end th method
        let statement=match self.connection.prep(query)       
        {
            Ok(data) => data,
            Err(err_msg) => 
            {
                log(&format!("Error on line {} in db_context: {}",line!(),err_msg));
                return None;
            },
        };   
        //execute the parametrized query with exec_first which will return one result as result<option<T>>
        let result=   self.connection.exec_first::<T, Statement, Params>(statement,params);
        // we used match pattern to extract the value if the value successfully extracted then store it in result var if it is not log the error and 
         let result=  match result 
        {
            Ok(data) => data,
            Err(err_msg) => 
            {
                log(&format!("Error on line {} in db context :{}",line!(),err_msg));
                return None;
            },
        };     
        result  
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
