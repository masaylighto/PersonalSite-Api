///this module have the method that deal with Skills Table
use super::{db_context::DbContext, logger::log,super::web_server::db_module};
use mysql::*;
use mysql::prelude::*;
use db_module::Skill;
impl DbContext 
{
    /// this method create a skill in the database 
    pub fn create_skill(self,name:&str,description:&str,category:&str)->bool
    {
         self.execute_parameterized("INSERT INTO SKILLS (NAME,DESCRIPTION,CATEGORY) values (:name,:disc,:cate)", params!("name"=>name,"disc"=>description,"cate"=>category))  
    }
    /// this method update the skill that specified by the id 
    pub fn edit_skill(self,name:&str,description:&str,category:&str,id:&str)->bool
    {
        self.execute_parameterized("UPDATE  SKILLS SET NAME=:name,DESCRIPTION=:disc,CATEGORY=:cate WHERE ID=:id", params!("name"=>name,"disc"=>description,"cate"=>category ,"id"=>id))  
    }
    /// this method DELETE the skill that specified by the id 
    pub fn delete_skill(self,id:&str)->bool
    {
       self.execute_parameterized("DELETE FROM  SKILLS WHERE ID=:id", params!("id"=>id))  
    }
    /// this method get the skills by there category 
    /// the return is result that contain vec that contain skill struct which represent the fields in skill table
    pub fn get_skill_by_category(mut self,category:&str)->Result<Vec<Skill>>
    {
          // this is a mapping function used to map the result of a select query into a skills struct
          let map_function=   |(id,name,description)|
          {
              db_module::Skill
              { 
              id: id, name: name, description: description,category:None
              }
          };
          // return vec of type 
         self.connection.exec_map("SELECT ID,NAME,description FROM SKILLS WHERE CATEGORY=:category", params!("category"=>category),map_function)  
    }
    /// this method get the categories in skill table
    /// the return is result that contain vec that contain the categories in skill table
    pub fn get_categories_list(mut self)->Result<Vec<Option<String>>>
    {
          // this is a mapping function used to map the result of a select query into a skills struct
          let map_function=   |category:Option<String>|
          {
            category
          };
          // return vec of type 
         self.connection.query_map("SELECT DISTINCT category FROM  SKILLS ",map_function)  
    }

}