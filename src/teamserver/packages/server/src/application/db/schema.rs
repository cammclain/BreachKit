use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::opt::Resource;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::Value;


// The operator is the user that can login to the teamserver. They can create and manage agents.
// They have a role that determines what they can do. The teamserver has a default operator with the root role.
#[derive(Debug, Serialize)]
struct Operator {
    id: RecordId,
    username: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    role: OperatorRole,
}

enum OperatorRole {
    Root,
    Admin,
    User,
}