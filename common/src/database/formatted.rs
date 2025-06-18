use crate::CommonError;

use super::raw::DbUserData;

#[derive(Debug, Clone)]
pub struct FormattedUserData {
    pub cid: i32,
    pub uid: i32,
}

impl FormattedUserData {
    pub fn check_if_full(value: &DbUserData) -> Result<FormattedUserData, CommonError> {
        match value.cid {
            Some(cid) => Ok(Self {
                cid,
                uid: value.uid.unwrap_or_default(),
            }),
            None => Err(
                "You havent fully Registered, create character on your account to complete process"
                    .into(),
            ),
        }
    }
}
