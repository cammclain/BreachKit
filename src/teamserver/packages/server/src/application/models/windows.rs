// This file contains the SurrealDB schema's for objects/observables that can be found in a Windows environment, within a larger Active Directory Environment
// This file contains schemas for detailed Active Directory and Windows system enumeration, similar to BloodHound

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// This model represents a Windows computer within an Active Directory environment
#[derive(Debug, Serialize, Deserialize)]
pub struct Computer {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub hostname: String,                // Computer hostname
    pub fqdn: Option<String>,           // Fully qualified domain name
    pub dns_name: Option<String>,       // DNS name
    pub sam_account_name: Option<String>, // SAM account name
    pub sid: Option<String>,            // Security Identifier
    pub operating_system: Option<String>, // OS name and version
    pub service_pack: Option<String>,    // Service pack level
    pub enabled: bool,                  // Whether account is enabled
    pub last_logon: Option<DateTime<Utc>>, // Last logon timestamp
    pub ip_addresses: Vec<String>,      // All associated IP addresses
    pub mac_addresses: Vec<String>,     // All associated MAC addresses
    pub domain: Option<Thing>,          // Domain this computer belongs to
    pub ou_path: Option<String>,        // OU path in AD
    pub local_groups: Vec<Thing>,       // Local groups on the system
    pub domain_groups: Vec<Thing>,      // Domain groups the computer belongs to
    pub logged_in_users: Vec<Thing>,    // Currently logged in users
    pub sessions: Vec<Thing>,           // Active sessions
    pub shares: Vec<Thing>,             // Network shares
    pub local_admins: Vec<Thing>,       // Local administrators
    pub remote_access: Vec<Thing>,      // Remote access configurations
    pub services: Vec<Thing>,           // Running services
    pub installed_software: Vec<Thing>, // Installed software
    pub patches: Vec<Thing>,            // Installed patches/updates
    pub credentials: Vec<Thing>,        // Discovered credentials
    pub trusts: Vec<Thing>,            // Trust relationships
    pub metadata: Option<serde_json::Value>, // Custom metadata
    pub first_seen: Option<DateTime<Utc>>, // First enumeration timestamp
    pub last_seen: Option<DateTime<Utc>>,  // Last enumeration timestamp
}

// Model for network shares found on computers
#[derive(Debug, Serialize, Deserialize)]
pub struct Share {
    pub id: Option<Thing>,              // SurrealDB record ID
    pub name: String,                   // Share name
    pub path: String,                   // Local path
    pub description: Option<String>,    // Share description
    pub permissions: Vec<Thing>,        // Share permissions
    pub computer: Thing,                // Computer hosting the share
    pub access_mask: Option<u32>,       // Access mask
    pub metadata: Option<serde_json::Value>, // Custom metadata
}

// Model for Windows services
#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<Thing>,              // SurrealDB record ID
    pub name: String,                   // Service name
    pub display_name: String,           // Display name
    pub description: Option<String>,    // Service description
    pub path: String,                   // Binary path
    pub start_type: String,             // Auto, Manual, Disabled
    pub service_type: String,           // Service type
    pub status: String,                 // Running, Stopped, etc.
    pub account: Option<Thing>,         // Service account
    pub computer: Thing,                // Host computer
    pub permissions: Vec<Thing>,        // Service permissions
    pub metadata: Option<serde_json::Value>, // Custom metadata
}

// Model for local groups on Windows systems
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalGroup {
    pub id: Option<Thing>,              // SurrealDB record ID
    pub name: String,                   // Group name
    pub sid: Option<String>,            // Security Identifier
    pub description: Option<String>,    // Group description
    pub members: Vec<Thing>,            // Group members
    pub computer: Thing,                // Computer where group exists
    pub privileges: Vec<String>,        // Group privileges
    pub metadata: Option<serde_json::Value>, // Custom metadata
}
