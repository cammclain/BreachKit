// This file contains the SurrealDB schema's for objects/observables that can be found in a Windows environment, within a larger Active Directory Environment
// This file contains thw surrealDB schemas for data that the agent can collect from windows environments within larger Active Directory environments.

use chrono::{Datetime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use surrealdb::RecordId;

// This model represents a hostname that a machine within the Active Directory environment has
#[derive(Debug, Serialize, Deserialize)]
struct Hostname<'a> {
    hostname_value: &'a str,
}

// This model represents a Windows host within an Active Directory environment
#[derive(Debug, Serialize)]
struct Host<'a> {
    id: RecordId,
    hostname: Hostname<'a>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Computer {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub hostname: String,                // Hostname
    pub ip_address: Option<String>,      // IP address
    pub os: Option<String>,              // Operating system
    pub groups: Vec<Thing>,              // IDs of groups the computer belongs to
    pub users: Vec<Thing>,               // IDs of users logged in
    pub credentials: Vec<Thing>,         // IDs of credentials found on the computer
    pub metadata: Option<serde_json::Value>, // Custom metadata
}
