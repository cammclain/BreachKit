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



#[derive(Debug, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Campaign name
    pub description: Option<String>,     // Campaign description
    pub objectives: Vec<Thing>,          // IDs of associated objectives
    pub metadata: Option<serde_json::Value>, // Custom metadata
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Objective {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Objective name
    pub description: Option<String>,     // Objective description
    pub campaign: Option<Thing>,         // Parent campaign ID
    pub goals: Vec<Thing>,               // IDs of related goals
    pub metadata: Option<serde_json::Value>, // Custom metadata
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Goal name
    pub description: Option<String>,     // Goal description
    pub objective: Option<Thing>,        // Parent objective ID
    pub status: String,                  // Status (e.g., "Pending", "Completed")
    pub metadata: Option<serde_json::Value>, // Custom metadata
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Loot {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Loot name
    pub type_: String,                   // Loot type (e.g., "file", "archive")
    pub location: String,                // Location (e.g., file path, URL)
    pub description: Option<String>,     // Description
    pub associated_entities: Vec<Thing>, // IDs of related entities
    pub metadata: Option<serde_json::Value>, // Custom metadata
}
