// The async_trait macro is essential for implementing async functions in traits
// This is necessary because Rust's traits don't natively support async functions
// We use this throughout our user model for database operations
use async_trait::async_trait;

// Import Local from chrono for timestamp handling
// Local represents the machine's local timezone
// We use this for tracking various timestamps like email verification and password resets
use chrono::offset::Local;

// Import core functionality from the loco_rs framework
// jwt: Provides JSON Web Token functionality for user authentication
// hash: Provides password hashing capabilities for secure password storage
// prelude: Contains commonly used traits and types from the framework
use loco_rs::{auth::jwt, hash, prelude::*};

// Import serialization traits from serde
// Deserialize: Allows converting from JSON/other formats into Rust structs
// Serialize: Allows converting Rust structs into JSON/other formats
// These are crucial for API request/response handling
use serde::{Deserialize, Serialize};

// Import UUID functionality for generating unique identifiers
// UUIDs are used throughout the application for:
// - User IDs (pid)
// - Verification tokens
// - Reset tokens
// - Other unique identifiers
use uuid::Uuid;

// Re-export the generated user entity types
// This makes the database-specific types available to other modules
// self: The users module itself
// ActiveModel: Used for creating/updating user records
// Entity: Represents the users table
// Model: Represents a user record
pub use super::_entities::users::{self, ActiveModel, Entity, Model};

// LoginParams struct defines the expected format for login requests
// This struct is used when users attempt to authenticate with the system
// It implements Debug for better error messages, and Serialize/Deserialize for API handling
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    // The user's email address
    // This serves as the unique identifier for login attempts
    // Must be a valid email format
    pub email: String,
    
    // The user's password in plain text
    // This will be verified against the hashed password stored in the database
    // Should never be stored or logged in plain text
    pub password: String,
}

// RegisterParams struct defines the required fields for new user registration
// This struct is used when creating new user accounts in the system
// Implements Debug for logging and Serialize/Deserialize for API handling
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    // The new user's email address
    // Must be unique in the system
    // Will be used for login and communication
    pub email: String,
    
    // The desired password for the new account
    // Will be hashed before storage
    // Should meet minimum security requirements
    pub password: String,
    
    // The user's display name
    // Used for personalization and display purposes
    // May appear in UI elements and communications
    pub name: String,
}

// Validator struct is used for input validation
// This ensures data integrity before any database operations
// Implements Debug for logging and Validate/Deserialize for validation processing
#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    // Name field validation
    // Ensures the name meets minimum length requirements
    // Custom validation message provides clear feedback to users
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,

    // Email field validation
    // Uses a custom validation function to ensure email format is correct
    // Prevents invalid email addresses from being stored in the database
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,
}

// Implementation of the Validatable trait for the user ActiveModel
// This connects our Validator struct with the database model
// Ensures all user data is validated before database operations
impl Validatable for super::_entities::users::ActiveModel {
    // Creates and returns a new Validator instance
    // Takes the current model's data and prepares it for validation
    // Returns a boxed trait object that can perform the validation
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            // Converts the name field to an owned String for validation
            name: self.name.as_ref().to_owned(),
            // Converts the email field to an owned String for validation
            email: self.email.as_ref().to_owned(),
        })
    }
}

// Implementation of ActiveModelBehavior for user models
// This trait handles lifecycle hooks for database operations
// Allows us to modify data before it's saved to the database
#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
    // before_save hook runs before any save operation
    // Handles both insert and update operations
    // Can modify the model data before it reaches the database
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // Validate the model data before saving
        // Ensures all business rules are met
        self.validate()?;

        // Special handling for new user insertions
        // Sets up required unique identifiers
        if insert {
            let mut this = self;
            // Generate and set a new UUID for the user
            // This becomes their permanent unique identifier
            this.pid = ActiveValue::Set(Uuid::new_v4());
            // Generate and set a unique API key
            // Prefixed with 'lo-' for identification
            this.api_key = ActiveValue::Set(format!("lo-{}", Uuid::new_v4()));
            Ok(this)
        } else {
            // For updates, return the model unchanged
            // Validation has already been performed
            Ok(self)
        }
    }
}

// Implementation of the Authenticable trait for user models
// Provides core authentication functionality
// Enables API key and claims-based authentication
#[async_trait]
impl Authenticable for super::_entities::users::Model {
    // Finds a user by their API key
    // Used for API authentication
    // Returns a Result containing the user or an error
    async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        // Query the database for a user with the matching API key
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        // Convert the Option<Model> to a Result
        // Returns EntityNotFound error if no user is found
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    // Finds a user by their claims key (typically their UUID)
    // Used for JWT authentication
    // Delegates to find_by_pid for actual lookup
    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_pid(db, claims_key).await
    }
}

// Implementation of additional methods for the user Model
// Provides various utility functions for user management
// Handles common user-related operations
impl super::_entities::users::Model {
    // Finds a user by their email address
    // Used for login and user lookup operations
    // Returns a Result containing the user or an error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        // Query the database for a user with the matching email
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, email)
                    .build(),
            )
            .one(db)
            .await?;
        // Convert the Option<Model> to a Result
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    // Finds a user by their email verification token
    // Used during the email verification process
    // Returns a Result containing the user or an error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        // Query the database for a user with the matching verification token
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::EmailVerificationToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        // Convert the Option<Model> to a Result
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    // Finds a user by their password reset token
    // Used during the password reset process
    // Returns a Result containing the user or an error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        // Query the database for a user with the matching reset token
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        // Convert the Option<Model> to a Result
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    // Finds a user by their permanent ID (UUID)
    // Used for general user lookup operations
    // Returns a Result containing the user or an error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: &str) -> ModelResult<Self> {
        // Parse the string UUID into a UUID type
        // Returns an error if the UUID is invalid
        let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        
        // Query the database for a user with the matching UUID
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Pid, parse_uuid)
                    .build(),
            )
            .one(db)
            .await?;
        // Convert the Option<Model> to a Result
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    // Finds a user by their API key
    // Used for API authentication
    // Returns a Result containing the user or an error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        // Query the database for a user with the matching API key
        let user = users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        // Convert the Option<Model> to a Result
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    // Verifies a user's password
    // Compares a plain text password against the stored hash
    // Returns true if the password matches, false otherwise
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        // Use the hash module to verify the password
        // Returns true if the hash of the provided password matches the stored hash
        hash::verify_password(password, &self.password)
    }

    // Creates a new user with a hashed password
    // Used during user registration
    // Returns a Result containing the new user or an error
    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &RegisterParams,
    ) -> ModelResult<Self> {
        // Start a database transaction
        // Ensures all operations succeed or fail together
        let txn = db.begin().await?;

        // Check if a user with the same email already exists
        // Prevents duplicate email addresses in the system
        if users::Entity::find()
            .filter(
                model::query::condition()
                    .eq(users::Column::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            // Return an error if the email is already in use
            return Err(ModelError::EntityAlreadyExists {});
        }

        // Hash the provided password for secure storage
        // Returns an error if hashing fails
        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        
        // Create a new user ActiveModel with the provided data
        let user = users::ActiveModel {
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            name: ActiveValue::set(params.name.to_string()),
            ..Default::default()
        }
        // Insert the new user into the database
        .insert(&txn)
        .await?;

        // Commit the transaction
        // Makes all changes permanent
        txn.commit().await?;

        // Return the newly created user
        Ok(user)
    }

    // Generates a JWT token for the user
    // Used for authentication and session management
    // Returns a Result containing the JWT string or an error
    pub fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        // Create a new JWT with the provided secret
        // Generate a token with the user's UUID as the subject
        Ok(jwt::JWT::new(secret).generate_token(expiration, self.pid.to_string(), None)?)
    }
}

// Implementation of additional methods for the user ActiveModel
// Provides functionality for user state management
// Handles email verification and password reset operations
impl super::_entities::users::ActiveModel {
    // Sets up email verification for a user
    // Generates a verification token and timestamp
    // Returns a Result containing the updated user or an error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<Model> {
        // Set the timestamp for when verification was sent
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().into()));
        // Generate and set a new verification token
        self.email_verification_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        // Update the user in the database
        Ok(self.update(db).await?)
    }

    // Sets up password reset for a user
    // Generates a reset token and timestamp
    // Returns a Result containing the updated user or an error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        // Set the timestamp for when the reset was sent
        self.reset_sent_at = ActiveValue::set(Some(Local::now().into()));
        // Generate and set a new reset token
        self.reset_token = ActiveValue::Set(Some(Uuid::new_v4().to_string()));
        // Update the user in the database
        Ok(self.update(db).await?)
    }

    // Marks a user's email as verified
    // Sets the verification timestamp
    // Returns a Result containing the updated user or an error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        // Set the timestamp for when the email was verified
        self.email_verified_at = ActiveValue::set(Some(Local::now().into()));
        // Update the user in the database
        Ok(self.update(db).await?)
    }

    // Resets a user's password
    // Hashes the new password and clears reset tokens
    // Returns a Result containing the updated user or an error
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<Model> {
        // Hash the new password for secure storage
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        // Clear the reset token
        self.reset_token = ActiveValue::Set(None);
        // Clear the reset timestamp
        self.reset_sent_at = ActiveValue::Set(None);
        // Update the user in the database
        Ok(self.update(db).await?)
    }
}
