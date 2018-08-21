use actix_web::{http::StatusCode, FromRequest, HttpResponse, Path, Query};
use bigneon_api::controllers::users;
use bigneon_api::controllers::users::{CurrentUser, PathParameters, SearchUserByEmail};
use bigneon_api::database::ConnectionGranting;
use bigneon_db::models::{DisplayUser, Roles, User};
use serde_json;
use support;
use support::database::TestDatabase;
use support::test_request::TestRequest;

#[test]
fn current_user() {
    let database = TestDatabase::new();
    let connection = database.get_connection();
    let db_user = User::create("Jeff", "Wilco", "test@test.com", "555-555-5555", "password")
        .commit(&*connection)
        .unwrap();

    let test_request = TestRequest::create(database);
    let state = test_request.extract_state();
    let user = support::create_auth_user_from_user(&db_user, Roles::Guest, &*connection);

    let response = users::current_user((state, user));

    assert_eq!(response.status(), StatusCode::OK);
    let body = support::unwrap_body_to_string(&response).unwrap();
    let cuser: CurrentUser = serde_json::from_str(&body).unwrap();
    let user = cuser.user;
    assert_eq!(user.first_name, "Jeff");
    assert_eq!(user.last_name, "Wilco");
    assert_eq!(user.id, db_user.id);
}

pub fn show_from_email(role: Roles, should_test_true: bool) {
    let database = TestDatabase::new();
    let connection = database.get_connection();
    let db_user = User::create("Jeff", "Last", "test@test.com", "555-555-5555", "password")
        .commit(&*connection)
        .unwrap();
    let test_request = TestRequest::create_with_uri(database, "/?email=test@test.com");
    let state = test_request.extract_state();
    let user = support::create_auth_user_from_user(&db_user, role, &*connection);
    let data = Query::<SearchUserByEmail>::from_request(&test_request.request, &()).unwrap();
    let response = users::find_by_email((state, data, user));
    let display_user: DisplayUser = db_user.into();
    let body = support::unwrap_body_to_string(&response).unwrap();

    if should_test_true {
        assert_eq!(response.status(), StatusCode::OK);
        let user: DisplayUser = serde_json::from_str(&body).unwrap();
        assert_eq!(user, display_user);
    } else {
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let temp_json = HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}));
        let event_expected_json = support::unwrap_body_to_string(&temp_json).unwrap();
        assert_eq!(body, event_expected_json);
    }
}

pub fn show(role: Roles, should_test_true: bool) {
    let database = TestDatabase::new();
    let connection = database.get_connection();
    let db_user = User::create("Jeff", "Last", "test@test.com", "555-555-5555", "password")
        .commit(&*connection)
        .unwrap();

    let db_display_user = User::find_by_email("test@test.com", &*connection)
        .unwrap()
        .unwrap()
        .for_display();

    let test_request = TestRequest::create(database);
    let state = test_request.extract_state();

    let user = support::create_auth_user(role, &*connection);
    let mut path = Path::<PathParameters>::extract(&test_request.request).unwrap();
    path.id = db_display_user.id;
    let response = users::show((state, path, user));
    let body = support::unwrap_body_to_string(&response).unwrap();

    if should_test_true {
        assert_eq!(response.status(), StatusCode::OK);
        let user: DisplayUser = serde_json::from_str(&body).unwrap();
        assert_eq!(user, db_display_user);
    } else {
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let temp_json = HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}));
        let event_expected_json = support::unwrap_body_to_string(&temp_json).unwrap();
        assert_eq!(body, event_expected_json);
    }
}