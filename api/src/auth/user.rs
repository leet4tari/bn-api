use actix_web::{HttpRequest, Result};
use bigneon_db::models::User as DbUser;
use bigneon_db::models::{Event, Organization, Roles, Scopes};
use bigneon_db::prelude::errors::EnumParseError;
use diesel::PgConnection;
use errors::*;
use log::Level::Warn;
use logging::*;
use serde_json::Value;
use server::AppState;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub user: DbUser,
    pub global_scopes: Vec<String>,
    pub ip_address: Option<String>,
    pub uri: String,
    pub method: String,
}

impl User {
    pub fn new(user: DbUser, request: &HttpRequest<AppState>) -> Result<User, EnumParseError> {
        let global_scopes = user
            .get_global_scopes()
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        Ok(User {
            user,
            global_scopes,
            ip_address: request.connection_info().remote().map(|i| i.to_string()),
            uri: request.uri().to_string(),
            method: request.method().to_string(),
        })
    }

    pub fn id(&self) -> Uuid {
        self.user.id
    }

    pub fn email(&self) -> Option<String> {
        self.user.email.clone()
    }

    fn check_scope_access(
        &self,
        scope: Scopes,
        organization: Option<&Organization>,
        event: Option<&Event>,
        connection: Option<&PgConnection>,
        log_on_failure: bool,
    ) -> Result<bool, BigNeonError> {
        if self.global_scopes.contains(&scope.to_string()) {
            return Ok(true);
        }

        let mut logging_data = HashMap::new();

        if let (Some(organization), Some(connection)) = (organization, connection) {
            let organization_scopes = organization.get_scopes_for_user(&self.user, connection)?;
            logging_data.insert("organization_scopes", json!(organization_scopes));
            logging_data.insert("organization_id", json!(organization.id));

            if organization_scopes.contains(&scope) {
                // User is an event limited access user so their organization event ids must include this event
                if event.is_some() {
                    // If the user's roles include an event limited role
                    let user_roles = organization.get_roles_for_user(&self.user, connection)?;
                    if Roles::get_event_limited_roles()
                        .iter()
                        .find(|r| user_roles.contains(&r))
                        .is_some()
                    {
                        if self
                            .user
                            .get_event_ids_for_organization(organization.id, connection)?
                            .contains(&event.unwrap().id)
                        {
                            return Ok(true);
                        }
                    } else {
                        return Ok(true);
                    }
                } else {
                    return Ok(true);
                }
            }
        }

        logging_data.insert("accessed_scope", json!(scope.to_string()));
        logging_data.insert("global_scopes", json!(self.global_scopes));

        if log_on_failure {
            self.log_unauthorized_access_attempt(logging_data);
        }
        Ok(false)
    }

    pub fn has_scope(&self, scope: Scopes) -> Result<bool, BigNeonError> {
        self.check_scope_access(scope, None, None, None, false)
    }

    pub fn has_scope_for_organization_event(
        &self,
        scope: Scopes,
        organization: &Organization,
        event: &Event,
        conn: &PgConnection,
    ) -> Result<bool, BigNeonError> {
        self.check_scope_access(scope, Some(organization), Some(event), Some(conn), false)
    }

    pub fn has_scope_for_organization(
        &self,
        scope: Scopes,
        organization: &Organization,
        conn: &PgConnection,
    ) -> Result<bool, BigNeonError> {
        self.check_scope_access(scope, Some(organization), None, Some(conn), false)
    }

    pub fn log_unauthorized_access_attempt(&self, mut logging_data: HashMap<&'static str, Value>) {
        logging_data.insert("user_id", json!(self.id()));
        logging_data.insert("user_name", json!(self.user.full_name()));
        logging_data.insert("ip_address", json!(self.ip_address));
        logging_data.insert("url", json!(self.uri));
        logging_data.insert("method", json!(self.method));
        jlog!(Warn, "Unauthorized access attempt", logging_data);
    }

    pub fn requires_scope(&self, scope: Scopes) -> Result<(), BigNeonError> {
        if self.check_scope_access(scope, None, None, None, true)? {
            return Ok(());
        }
        Err(AuthError::new(
            AuthErrorType::Unauthorized,
            "User does not have the required permissions".to_string(),
        )
        .into())
    }

    pub fn requires_scope_for_organization_event(
        &self,
        scope: Scopes,
        organization: &Organization,
        event: &Event,
        conn: &PgConnection,
    ) -> Result<(), BigNeonError> {
        if self.check_scope_access(scope, Some(organization), Some(event), Some(conn), true)? {
            return Ok(());
        }
        Err(AuthError::new(
            AuthErrorType::Unauthorized,
            "User does not have the required permissions".to_string(),
        )
        .into())
    }

    pub fn requires_scope_for_organization(
        &self,
        scope: Scopes,
        organization: &Organization,
        conn: &PgConnection,
    ) -> Result<(), BigNeonError> {
        if self.check_scope_access(scope, Some(organization), None, Some(conn), true)? {
            return Ok(());
        }
        Err(AuthError::new(
            AuthErrorType::Unauthorized,
            "User does not have the required permissions".to_string(),
        )
        .into())
    }
}
