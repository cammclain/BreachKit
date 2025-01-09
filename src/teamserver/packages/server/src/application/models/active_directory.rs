// This file is for the SurrealDB database models/schema for the various Active Directory observables that the agents within the BreachKit framework can collect

// This model represents a Domain within the BreachKit framework
#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Domain name (e.g., "contoso.local")
    pub forest_name: Option<String>,     // Forest name if part of a forest
    pub domain_controllers: Vec<Thing>,   // IDs of domain controllers
    pub trusts: Vec<Thing>,              // IDs of trusted domains
    pub functional_level: Option<String>, // Domain functional level
    pub metadata: Option<serde_json::Value>, // Custom metadata
}

// This model represents a credential within the BreachKit framework
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub username: String,                // Username of the credential
    pub password: Option<String>,        // Password (if plaintext)
    pub hash: Option<String>,            // Password hash
    pub hash_type: Option<String>,       // Type of hash (e.g., "NTLM", "Kerberos")
    pub domain: Option<Thing>,           // Domain this credential belongs to
    pub discovered_on: Option<Thing>,    // Computer where credential was found
    pub discovered_at: Option<DateTime<Utc>>, // When the credential was discovered
    pub metadata: Option<serde_json::Value>,  // Custom metadata
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<Thing>,               // SurrealDB record ID
    pub name: String,                    // Group name
    pub sid: Option<String>,             // Security Identifier
    pub scope: String,                   // Domain local, Global, or Universal
    pub group_type: String,              // Security or Distribution
    pub members: Vec<Thing>,             // IDs of members (users/computers)
    pub member_of: Vec<Thing>,           // Groups this group is a member of
    pub privileges: Vec<String>,         // Privileges granted by the group
    pub description: Option<String>,     // Group description
    pub domain: Thing,                   // Domain this group belongs to
    pub created_at: Option<DateTime<Utc>>, // Creation timestamp
    pub modified_at: Option<DateTime<Utc>>, // Last modification timestamp
    pub metadata: Option<serde_json::Value>, // Custom metadata
}
