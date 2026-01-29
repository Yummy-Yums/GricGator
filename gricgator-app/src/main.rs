use gricgator_app::{get_avail_locations_data, list_categories_of_commodities};

#[tokio::main]
async fn main() {

   let message = r#"
Welcome to Gricgator!
Please use the cli command to access program, Thanks
   "#;
    println!("{}", message)
}