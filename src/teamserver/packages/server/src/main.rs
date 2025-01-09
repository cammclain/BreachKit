// This is of course the main.rs for the application, and handles the ove3rall "launching" of the application.
// It will pull in all the functionality of the application from the application module.

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
}
