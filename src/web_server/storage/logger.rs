/// this page used to represent logging method
/// this is a global log method all class method that need to log data will call this method 
pub fn log(log_information:&String)
{
    println!("{}",&log_information);    
}