use actix_session::Session;

pub fn check_auth(session: &Session) -> bool {
    session.get::<String>("username").unwrap_or(None).is_some()
}

pub fn set_session(session: &Session, username: &str) -> Result<(), actix_session::SessionInsertError> {
    session.insert("username", username.to_string())
}

pub fn clear_session(session: &Session) {
    session.purge();
}

