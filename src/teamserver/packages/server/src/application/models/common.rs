// This file contains thw surrealDB schemas for data that the agent can collect from windows environments within larger Active Directory environments.

use serde::{Deserialize, Serialize};

use chrono::{Datetime, Utc};

use surrealdb::sql::Thing;

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
