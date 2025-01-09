// This file contains the database client which connects to the SurrealDB database using Websocket connection

use std::sync::LazyLock;

// TODO: Not 100% sure but I think I will be better off if I put this in the database_setup function
static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

// This function is publicly exported for setting up the database when the application is initialized
// The idea is that using the LazyLock feature we can avoid reinitialization of the DB value.
// This means we are able to avoid opening and closing new connections and always have a thread safe way to read/write to the database
// it will preform all the setups required and return the DB value using LazyLock 
pub fn database_setup() -> DB {
    println!("Rust on iphone :P")    
}

