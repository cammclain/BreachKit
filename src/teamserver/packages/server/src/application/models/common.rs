// This file contains the SurrealDB schema's for objects/observables that are a part of the framework itself
// It also contains the 
use serde::{Deserialize, Serialize};

use chrono::{Datetime, Utc};

use surrealdb::sql::Thing;

// This model represents a user/operator within the BreachKit framework
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,     // SurrealDB record ID
    pub username: String,      // AD username
    pub email: Option<String>, // the operator's email address
    //pub role: Vec<Thing>, TODO: add a Role enum with like admin, modertor? or some mid range user, then just a standard user role.
    pub groups: Vec<Thing>,                // the groups the user belongs to
    pub credentials: Vec<Thing>, // IDs of the related credentials? NOTE: I am honestly not sure if I want to link "credentials" to the campagin or like the actual operation
    pub last_login: Option<DateTime<Utc>>, // last login timestamp
}


// This model represents a credential within the BreachKit framework
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: Option<Thing>, // SurrealDB record ID
    pub username: String, // the username of the credential
    pub password: String, // the password of the credential
}
        