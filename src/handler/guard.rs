use actix_session::{Session, UserSession};
use actix_web::dev::RequestHead;
use actix_web::guard::Guard;
use log::info;

pub const UID_TAG: &str = "uid";

pub struct SessionGuard;

impl Guard for SessionGuard {
    fn check(&self, request: &RequestHead) -> bool {
        let session: Session = request.get_session();
        let user = session.get::<String>(UID_TAG).unwrap();
        info!("user {:?}", user);
        user.is_some()
    }
}

#[allow(dead_code)] 
pub fn get_uid(session: Session) -> String {
    session.get::<String>(UID_TAG).unwrap().unwrap()
}

pub fn set_uid(session: Session, uid: String) {
    session.set(UID_TAG, uid).unwrap()
}
