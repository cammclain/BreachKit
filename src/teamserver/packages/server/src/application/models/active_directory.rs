// This file is for the SurrealDB database models/schema for the various Active Directory observables that the agents within the BreachKit framework can collect

// This model represents a Domain within the BreachKit framework
#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub id: Option<Thing>, // SurrealDB record ID
    pub name: String, // the name of the domain
}

// This model represents a credential within the BreachKit framework
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: Option<Thing>, // SurrealDB record ID
    pub username: String, // the username of the credential
    pub password: String, // the password of the credential
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Group name
    pub members: Vec<Thing>,             // IDs of members (users/computers)
    pub privileges: Vec<String>,         // Privileges granted by the group
    pub metadata: Option<serde_json::Value>, // Custom metadata
}
