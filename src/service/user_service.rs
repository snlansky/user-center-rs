use crate::model::User;
use crate::error::Result;
pub struct UserService{

}

impl UserService {
    pub fn new() -> Self {
        Self{}
    }

    pub fn save(u :User) -> Result<()> {
        Ok(())
    }
}