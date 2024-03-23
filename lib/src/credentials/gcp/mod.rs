use super::{Credentials, CredentialsError};
use error_stack::{Report, ResultExt};
pub use gcp_auth::AuthenticationManager as GcpCredentials;

impl Credentials for GcpCredentials {
    async fn get_access_token(&self, scopes: &[&str]) -> Result<String, Report<CredentialsError>> {
        let token = self
            .get_token(scopes)
            .await
            .change_context(CredentialsError::Internal)?;

        Ok(token.as_str().into())
    }
}
