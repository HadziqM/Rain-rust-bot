use super::raw::DbUserData;

pub struct FormattedUserData {
    pub cid: i32,
    pub uid: i32,
}

impl TryFrom<DbUserData> for FormattedUserData {
    type Error = crate::CommonError;

    fn try_from(value: DbUserData) -> Result<Self, Self::Error> {
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
