// This file handles the authentication of the clients when they connect to the team server

// TODO: "import" the struct that defines the credentials for the client
// TODO: import the ClientAuthToken struct 



// authenticate_client is a function that provides a client that submits valid credentials with some sort of Session Token 
pub async fn authenticate_client(client_login_credentials) -> ClientAuthToken {
    // TODO: Implement the authenticate_client function
    // NOTE: this "client auth token" needs to be generated
    println!("TODO implemebnt the fucking authentyicatee client shiot ")
}

// TODO: this is clearly disgustingly fucked up but i am thinking we have it delete the ClientAuthToken from the database? idk.
pub async fn logout_client(active_client_auth_token) -> Result() {
    println!("TODO IMPLEMENT THE logout_client function in server/src/application/handlers/auth.rs")   
}
