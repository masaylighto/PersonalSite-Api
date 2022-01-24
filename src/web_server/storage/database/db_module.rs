#[derive(Debug)]
pub struct USER
{

   pub id:i32,
   pub name:Option<String>,
   pub username:Option<String>,    
   pub email:Option<String>,
   pub password:Option<String>,
   pub is_owner:bool
}