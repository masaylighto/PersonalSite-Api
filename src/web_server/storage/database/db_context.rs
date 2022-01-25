/// this module hold the responsiblity to deal with the database
use mysql::*;
use mysql::prelude::*;
use super::loger::log;
mod db_module;
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
       /// used to excute sql query with parameters
       pub fn excute_parameterized(mut self,query:&str,params:Params)->bool
       {
           //create a Paramiterized query this method will return option so we will use match pattren and if it did not successed then we end th method
           let statement=match self.connection.prep(query)       
           {
               Ok(data) => data,
               Err(err_msg) => 
               {
                   log(&format!("Error on line {} in db_context: {}",line!(),err_msg));
                   return false;
               },
           };   
           //excute the paramitrized query with exec_drop which will return result struct with no data 
           // we will use result to detrimine if the excution was completed successfully
           let result=   self.connection.exec_drop(statement,params);
           //the result come as Result  so we will use method Is_ok as return cause if the query excuted successfully the value will be true
           // and that what we will return and if the query falied to excute then it will return false and that what we will return
           result.is_ok()
       }
    /// used to excute sql query and get one cell of data
    pub fn get_cell<T:FromRow>(mut self,query:&str,params:Params)->Option<T>
    {
        //create a Paramiterized query this method will return option so we will use match pattren and if it did not successed then we end th method
        let statement=match self.connection.prep(query)       
        {
            Ok(data) => data,
            Err(err_msg) => 
            {
                log(&format!("Error on line {} in db_context: {}",line!(),err_msg));
                return None;
            },
        };   
        //excute the paramitrized query with exec_first which will return one result as result<option<T>>
        let result=   self.connection.exec_first::<T, Statement, Params>(statement,params);
        // we used match pattren to extact the value if the value successfuly exctracted then store it in result var if it is not log the error and 
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
    /// this method check if user is exist 
    pub fn is_user_exist(self,username:&str,password:&str)->bool
    {
        // get the count of user that match the information provided by the user
        let  result= self.get_cell::<i32>("SELECT COUNT(*) FROM ACCOUNTS WHERE USERNAME=:user_name AND PASSWORD=:pass_word",params!("user_name"=>username,"pass_word"=>password));       
        // return false to the caller as indicator that checking user info didnt go well and to prevent the system from considering the information as right and login unauthorized user
        let result = match result
        {
            Some(data) => data,
            None =>
            {
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
    /// this method will create a new user in the database
    pub fn create_user(self,name:&str,username:&str,email:&str,password:&str,is_owner:&str)->bool
    {
       // insert the new user information into the database
       let  is_successed= self.excute_parameterized("INSERT INTO ACCOUNTS (NAME,USERNAME,EMAIL,PASSWORD,ISOWNER) VALUES(:name,:user_name,:email,:pass_word,:is_owner)",params!("name"=>name,"user_name"=>username,"email"=>email,"pass_word"=>password,"is_owner"=>is_owner));       
       // return the result
       is_successed    
    }
    /// this method will update the user specify by the id in the database
    pub fn update_user(self,id:&str,name:&str,username:&str,email:&str,password:&str,is_owner:&str)->bool
    {
        // update the user information in the database
        let  is_successed= self.excute_parameterized("UPDATE ACCOUNTS SET NAME=:name,USERNAME=:user_name,EMAIL=:email,PASSWORD=:pass_word,ISOWNER=:is_owner WHERE ID=:id",params!("name"=>name,"user_name"=>username,"email"=>email,"pass_word"=>password,"is_owner"=>is_owner,"id"=>id));       
        // return the result
        is_successed    
    }    
     /// this method will delete the user specify by the id from the database ,if the user wasn't an admin
     pub fn delete_user(self,id:&str)->bool
     {
         // update the user information in the database
         let  is_successed= self.excute_parameterized("DELETE FROM ACCOUNTS WHERE ID=:id and ISOWNER=0",params!("id"=>id));       
         // return the result
         is_successed    
     }   
     /// this method will get all users or specific count of user as Vec of Struct User in db_module
     /// if the limit paramter is None then the method will get all user
     /// and if it is not it will extact it value and specify the amont of row it will select
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
        // we will preforme a select query and map the result to a struct name User in db_module 
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
    ///get user by id  , this method will return vec that contain the user info in its first index
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
        // we will preforme a select query and map the result to a struct name User in db_module 
        // the function result will be  result struct that contain a vec of the type User that will have the user info as user struct at it first index
        self.connection.exec_map("SELECT  ID,NAME,USERNAME,EMAIL,ISOWNER FROM ACCOUNTS where id=:id ",params!("id"=>id),map_function)    
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