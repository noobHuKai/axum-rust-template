use std::sync::Arc;

use casbin::CoreApi;

use crate::error::AuthError;

pub fn check_permissions(
    enforcer: Arc<casbin::Enforcer>,
    role: String,
    uri: &str,
    method: &str,
) -> Result<(), AuthError> {
    if let Ok(authorized) = enforcer.enforce((role, uri, method)) {
        if authorized {
            Ok(())
        } else {
            // deny the request
            Err(AuthError::WrongCredentials)
        }
    } else {
        // error occurs
        Err(AuthError::InternalServerError)
    }
}
