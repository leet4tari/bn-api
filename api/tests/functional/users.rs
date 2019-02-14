use actix_web::{http::StatusCode, HttpResponse};
use bigneon_api::auth::TokenResponse;
use bigneon_api::controllers::users;
use bigneon_api::extractors::*;
use bigneon_api::models::{RegisterRequest, UserProfileAttributes};
use bigneon_db::prelude::*;
use functional::base;
use serde_json;
use std::collections::HashMap;
use support;
use support::database::TestDatabase;
use support::test_request::TestRequest;

use bigneon_api::errors::BigNeonError;

#[cfg(test)]
mod history_tests {
    use super::*;
    #[test]
    fn history_org_member() {
        base::users::history(Roles::OrgMember, true);
    }
    #[test]
    fn history_admin() {
        base::users::history(Roles::Admin, true);
    }
    #[test]
    fn history_user() {
        base::users::history(Roles::User, false);
    }
    #[test]
    fn history_org_owner() {
        base::users::history(Roles::OrgOwner, true);
    }
    #[test]
    fn history_door_person() {
        base::users::history(Roles::DoorPerson, false);
    }
    #[test]
    fn history_promoter() {
        base::users::history(Roles::Promoter, false);
    }
    #[test]
    fn history_promoter_read_only() {
        base::users::history(Roles::PromoterReadOnly, false);
    }
    #[test]
    fn history_org_admin() {
        base::users::history(Roles::OrgAdmin, true);
    }
    #[test]
    fn history_box_office() {
        base::users::history(Roles::OrgBoxOffice, false);
    }
}

#[cfg(test)]
mod profile_tests {
    use super::*;
    #[test]
    fn profile_org_member() {
        base::users::profile(Roles::OrgMember, true);
    }
    #[test]
    fn profile_admin() {
        base::users::profile(Roles::Admin, true);
    }
    #[test]
    fn profile_user() {
        base::users::profile(Roles::User, false);
    }
    #[test]
    fn profile_org_owner() {
        base::users::profile(Roles::OrgOwner, true);
    }
    #[test]
    fn profile_door_person() {
        base::users::profile(Roles::DoorPerson, false);
    }
    #[test]
    fn profile_promoter() {
        base::users::profile(Roles::Promoter, false);
    }
    #[test]
    fn profile_promoter_read_only() {
        base::users::profile(Roles::PromoterReadOnly, false);
    }
    #[test]
    fn profile_org_admin() {
        base::users::profile(Roles::OrgAdmin, true);
    }
    #[test]
    fn profile_box_office() {
        base::users::profile(Roles::OrgBoxOffice, false);
    }
}

#[cfg(test)]
mod user_list_organizations_tests {
    use super::*;
    #[test]
    fn list_organizations_org_member() {
        base::users::list_organizations(Roles::OrgMember, false);
    }
    #[test]
    fn list_organizations_admin() {
        base::users::list_organizations(Roles::Admin, true);
    }
    #[test]
    fn list_organizations_user() {
        base::users::list_organizations(Roles::User, false);
    }
    #[test]
    fn list_organizations_org_owner() {
        base::users::list_organizations(Roles::OrgOwner, false);
    }
    #[test]
    fn list_organizations_door_person() {
        base::users::list_organizations(Roles::DoorPerson, false);
    }
    #[test]
    fn list_organizations_promoter() {
        base::users::list_organizations(Roles::Promoter, false);
    }
    #[test]
    fn list_organizations_promoter_read_only() {
        base::users::list_organizations(Roles::PromoterReadOnly, false);
    }
    #[test]
    fn list_organizations_org_admin() {
        base::users::list_organizations(Roles::OrgAdmin, false);
    }
    #[test]
    fn list_organizations_box_office() {
        base::users::list_organizations(Roles::OrgBoxOffice, false);
    }
}

#[cfg(test)]
mod show_push_notification_tokens_for_user_id_tests {
    use super::*;
    #[test]
    fn show_push_notification_tokens_for_user_id_org_member() {
        base::users::show_push_notification_tokens_for_user_id(Roles::OrgMember, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_admin() {
        base::users::show_push_notification_tokens_for_user_id(Roles::Admin, true);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_user() {
        base::users::show_push_notification_tokens_for_user_id(Roles::User, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_org_owner() {
        base::users::show_push_notification_tokens_for_user_id(Roles::OrgOwner, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_door_person() {
        base::users::show_push_notification_tokens_for_user_id(Roles::DoorPerson, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_promoter() {
        base::users::show_push_notification_tokens_for_user_id(Roles::Promoter, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_promoter_read_only() {
        base::users::show_push_notification_tokens_for_user_id(Roles::PromoterReadOnly, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_org_admin() {
        base::users::show_push_notification_tokens_for_user_id(Roles::OrgAdmin, false);
    }
    #[test]
    fn show_push_notification_tokens_for_user_id_box_office() {
        base::users::show_push_notification_tokens_for_user_id(Roles::OrgBoxOffice, false);
    }
}

#[cfg(test)]
mod show_push_notification_tokens_tests {
    use super::*;
    #[test]
    fn show_push_notification_tokens_org_member() {
        base::users::show_push_notification_tokens(Roles::OrgMember, true);
    }
    #[test]
    fn show_push_notification_tokens_admin() {
        base::users::show_push_notification_tokens(Roles::Admin, true);
    }
    #[test]
    fn show_push_notification_tokens_user() {
        base::users::show_push_notification_tokens(Roles::User, true);
    }
    #[test]
    fn show_push_notification_tokens_org_owner() {
        base::users::show_push_notification_tokens(Roles::OrgOwner, true);
    }
    #[test]
    fn show_push_notification_tokens_door_person() {
        base::users::show_push_notification_tokens(Roles::DoorPerson, true);
    }
    #[test]
    fn show_push_notification_tokens_promoter() {
        base::users::show_push_notification_tokens(Roles::Promoter, true);
    }
    #[test]
    fn show_push_notification_tokens_promoter_read_only() {
        base::users::show_push_notification_tokens(Roles::PromoterReadOnly, true);
    }
    #[test]
    fn show_push_notification_tokens_org_admin() {
        base::users::show_push_notification_tokens(Roles::OrgAdmin, true);
    }
    #[test]
    fn show_push_notification_tokens_box_office() {
        base::users::show_push_notification_tokens(Roles::OrgBoxOffice, true);
    }
}

#[cfg(test)]
mod add_push_notification_token_tests {
    use super::*;
    #[test]
    fn add_push_notification_token_org_member() {
        base::users::add_push_notification_token(Roles::OrgMember, true);
    }
    #[test]
    fn add_push_notification_token_admin() {
        base::users::add_push_notification_token(Roles::Admin, true);
    }
    #[test]
    fn add_push_notification_token_user() {
        base::users::add_push_notification_token(Roles::User, true);
    }
    #[test]
    fn add_push_notification_token_org_owner() {
        base::users::add_push_notification_token(Roles::OrgOwner, true);
    }
    #[test]
    fn add_push_notification_token_door_person() {
        base::users::add_push_notification_token(Roles::DoorPerson, true);
    }
    #[test]
    fn add_push_notification_token_promoter() {
        base::users::add_push_notification_token(Roles::Promoter, true);
    }
    #[test]
    fn add_push_notification_token_promoter_read_only() {
        base::users::add_push_notification_token(Roles::PromoterReadOnly, true);
    }
    #[test]
    fn add_push_notification_token_org_admin() {
        base::users::add_push_notification_token(Roles::OrgAdmin, true);
    }
    #[test]
    fn add_push_notification_token_box_office() {
        base::users::add_push_notification_token(Roles::OrgBoxOffice, true);
    }
}

#[cfg(test)]
mod remove_push_notification_token_tests {
    use super::*;
    #[test]
    fn remove_push_notification_token_org_member() {
        base::users::remove_push_notification_token(Roles::OrgMember, true);
    }
    #[test]
    fn remove_push_notification_token_admin() {
        base::users::remove_push_notification_token(Roles::Admin, true);
    }
    #[test]
    fn remove_push_notification_token_user() {
        base::users::remove_push_notification_token(Roles::User, true);
    }
    #[test]
    fn remove_push_notification_token_owner() {
        base::users::remove_push_notification_token(Roles::OrgOwner, true);
    }
    #[test]
    fn remove_push_notification_token_door_person() {
        base::users::remove_push_notification_token(Roles::DoorPerson, true);
    }
    #[test]
    fn remove_push_notification_token_promoter() {
        base::users::remove_push_notification_token(Roles::Promoter, true);
    }
    #[test]
    fn remove_push_notification_token_promoter_read_only() {
        base::users::remove_push_notification_token(Roles::PromoterReadOnly, true);
    }
    #[test]
    fn remove_push_notification_token_org_admin() {
        base::users::remove_push_notification_token(Roles::OrgAdmin, true);
    }
    #[test]
    fn remove_push_notification_token_box_office() {
        base::users::remove_push_notification_token(Roles::OrgBoxOffice, true);
    }
}

#[cfg(test)]
mod users_show_tests {
    use super::*;
    #[test]
    fn show_org_member() {
        base::users::show(Roles::OrgMember, false);
    }
    #[test]
    fn show_admin() {
        base::users::show(Roles::Admin, true);
    }
    #[test]
    fn show_user() {
        base::users::show(Roles::User, false);
    }
    #[test]
    fn show_org_owner() {
        base::users::show(Roles::OrgOwner, false);
    }
    #[test]
    fn show_door_person() {
        base::users::show(Roles::DoorPerson, false);
    }
    #[test]
    fn show_promoter() {
        base::users::show(Roles::Promoter, false);
    }
    #[test]
    fn show_promoter_read_only() {
        base::users::show(Roles::PromoterReadOnly, false);
    }
    #[test]
    fn show_org_admin() {
        base::users::show(Roles::OrgAdmin, false);
    }
    #[test]
    fn show_box_office() {
        base::users::show(Roles::OrgBoxOffice, false);
    }
}

#[test]
fn register_address_exists() {
    let database = TestDatabase::new();
    let request = TestRequest::create();
    let existing_user = database.create_user().finish();

    let json = Json(RegisterRequest::new(
        &"First",
        &"Last",
        &existing_user.email.unwrap(),
        &"555",
        &"not_important",
        None,
    ));

    let response: HttpResponse =
        users::register((request.request, database.connection.into(), json)).into();

    if response.status() == StatusCode::OK {
        panic!("Duplicate email was allowed when it should not be")
    }
}

#[test]
fn register_succeeds_without_name() {
    let database = TestDatabase::new();
    let request = TestRequest::create();
    let json = Json(RegisterRequest {
        email: "noname@localhost".to_string(),
        password: "password".to_string(),
        first_name: None,
        last_name: None,
        phone: None,
        captcha_response: None,
    });

    let response: HttpResponse =
        users::register((request.request, database.connection.into(), json)).into();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[test]
fn register_succeeds() {
    let database = TestDatabase::new();
    let request = TestRequest::create();
    let json = Json(RegisterRequest::new(
        &"First",
        &"Last",
        &"fake@localhost",
        &"555",
        &"not_important",
        None,
    ));

    let response: HttpResponse =
        users::register((request.request, database.connection.into(), json)).into();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[test]
fn register_succeeds_with_login() {
    let database = TestDatabase::new();
    let request = TestRequest::create();
    let json = Json(RegisterRequest::new(
        &"First",
        &"Last",
        &"fake@localhost",
        &"555",
        &"not_important",
        None,
    ));

    let response: HttpResponse =
        users::register_and_login((request.request, database.connection.into(), json)).into();
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = support::unwrap_body_to_string(&response).unwrap();
    let token_response: TokenResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(token_response.access_token.is_empty(), false);
    assert_eq!(token_response.refresh_token.is_empty(), false);
}

#[test]
fn register_with_validation_errors() {
    let database = TestDatabase::new();
    let request = TestRequest::create();
    let json = Json(RegisterRequest::new(
        &"First",
        &"Last",
        &"bad-email",
        &"555",
        &"not_important",
        None,
    ));

    let response: HttpResponse =
        users::register((request.request, database.connection.into(), json)).into();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.error().is_some());

    let validation_response = support::validation_response_from_response(&response).unwrap();
    let email = validation_response.fields.get("email").unwrap();
    assert_eq!(email[0].code, "email");
    assert_eq!(
        &email[0].message.clone().unwrap().into_owned(),
        "Email is invalid"
    );
}

#[test]
fn current_user() {
    let database = TestDatabase::new();
    let user = database.create_user().finish();
    let auth_user = support::create_auth_user_from_user(&user, Roles::User, None, &database);

    let response = users::current_user((database.connection.into(), auth_user)).unwrap();
    let user = response.user;
    assert_eq!(user.id, user.id);
    assert_eq!(
        vec![
            Scopes::EventInterest,
            Scopes::OrderReadOwn,
            Scopes::TicketTransfer
        ],
        response.scopes
    );
    assert!(response.organization_scopes.is_empty());
}

#[test]
fn current_user_organization_owner() {
    let database = TestDatabase::new();
    let user = database.create_user().finish();
    let organization = database.create_organization().finish();
    let auth_user =
        support::create_auth_user_from_user(&user, Roles::OrgOwner, Some(&organization), &database);

    let current_user =
        users::current_user((database.connection.clone().into(), auth_user)).unwrap();
    let user = current_user.user;
    assert_eq!(user.id, user.id);
    let mut expected_results = HashMap::new();
    expected_results.insert(
        organization.id,
        vec![
            "artist:write",
            "box-office-ticket:read",
            "box-office-ticket:write",
            "code:read",
            "code:write",
            "comp:read",
            "comp:write",
            "dashboard:read",
            "event:cancel",
            "event:financial-reports",
            "event:interest",
            "event:reports",
            "event:scan",
            "event:view-guests",
            "event:write",
            "hold:read",
            "hold:write",
            "order:make-external-payment",
            "order:read",
            "order:read-own",
            "order:refund",
            "org:admin-users",
            "org:fans",
            "org:read",
            "org:reports",
            "org:users",
            "org:write",
            "redeem:ticket",
            "ticket:admin",
            "ticket:read",
            "ticket:transfer",
            "ticket-type:read",
            "ticket-type:write",
            "user:read",
            "venue:write",
        ]
        .into_iter()
        .map(|scope| scope.parse::<Scopes>().unwrap())
        .collect(),
    );
    assert_eq!(expected_results, current_user.organization_scopes);

    let mut expected_roles = HashMap::new();
    expected_roles.insert(organization.id, vec![Roles::OrgOwner]);
    assert_eq!(expected_roles, current_user.organization_roles);
}

#[test]
fn current_user_organization_member() {
    let database = TestDatabase::new();
    let user = database.create_user().finish();
    let organization = database.create_organization().finish();
    let auth_user = support::create_auth_user_from_user(
        &user,
        Roles::OrgMember,
        Some(&organization),
        &database,
    );

    let current_user =
        users::current_user((database.connection.clone().into(), auth_user)).unwrap();
    let user = current_user.user;
    assert_eq!(user.id, user.id);
    assert_eq!(
        vec![
            Scopes::EventInterest,
            Scopes::OrderReadOwn,
            Scopes::TicketTransfer
        ],
        current_user.scopes
    );
    let mut expected_scopes = HashMap::new();
    expected_scopes.insert(
        organization.id,
        vec![
            "artist:write",
            "box-office-ticket:read",
            "box-office-ticket:write",
            "code:read",
            "code:write",
            "comp:read",
            "comp:write",
            "dashboard:read",
            "event:cancel",
            "event:interest",
            "event:scan",
            "event:view-guests",
            "event:write",
            "hold:read",
            "hold:write",
            "order:read",
            "order:read-own",
            "order:refund",
            "org:fans",
            "org:read",
            "redeem:ticket",
            "ticket:admin",
            "ticket:read",
            "ticket:transfer",
            "ticket-type:read",
            "ticket-type:write",
            "venue:write",
        ]
        .into_iter()
        .map(|scope| scope.parse::<Scopes>().unwrap())
        .collect(),
    );
    assert_eq!(expected_scopes, current_user.organization_scopes);

    let mut expected_roles = HashMap::new();
    expected_roles.insert(organization.id, vec![Roles::OrgMember]);
    assert_eq!(expected_roles, current_user.organization_roles);
}

#[test]
pub fn update_current_user() {
    let database = TestDatabase::new();
    let user = support::create_auth_user(Roles::User, None, &database);
    let email = "new-email@tari.com";
    let mut attributes: UserProfileAttributes = Default::default();
    attributes.email = Some(Some(email.to_string()));
    let json = Json(attributes);

    let updated_user =
        users::update_current_user((database.connection.into(), json, user)).unwrap();
    assert_eq!(updated_user.user.email, Some(email.into()));
}

#[test]
pub fn update_current_user_with_validation_errors() {
    let database = TestDatabase::new();
    let user = support::create_auth_user(Roles::User, None, &database);
    let mut attributes: UserProfileAttributes = Default::default();
    attributes.email = Some(Some("bad-email".into()));
    let json = Json(attributes);

    let result: Result<HttpResponse, BigNeonError> =
        Err(
            users::update_current_user((database.connection.into(), json, user))
                .err()
                .unwrap(),
        );

    let response = result.into();
    let validation_response = support::validation_response_from_response(&response).unwrap();
    let email = validation_response.fields.get("email").unwrap();
    assert_eq!(email[0].code, "email");
    assert_eq!(
        &email[0].message.clone().unwrap().into_owned(),
        "Email is invalid"
    );
}

#[test]
fn update_current_user_address_exists() {
    let database = TestDatabase::new();
    let existing_user = database.create_user().finish();

    let user = support::create_auth_user(Roles::User, None, &database);
    let mut attributes: UserProfileAttributes = Default::default();
    attributes.email = Some(existing_user.email);
    let json = Json(attributes);

    let result: Result<HttpResponse, BigNeonError> =
        Err(
            users::update_current_user((database.connection.into(), json, user))
                .err()
                .unwrap(),
        );
    let response: HttpResponse = result.into();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}
