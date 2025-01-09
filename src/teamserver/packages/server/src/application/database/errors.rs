mod error {
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::Json;
    use thiserror::Error;

    // Define the error types
    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Database Error")]
        Db,
    }

    // Implement the IntoResponse trait for the Error type
    impl IntoResponse for Error {
        fn into_response(self) -> Response {
    }
}