use crate::utility::error::Result;

pub trait Handler: Send + Sync {
    // check user exists
    fn exists(&self, user_id: &String) -> Result<bool>;

    // get all users
    fn users(&self) -> Result<Vec<String>>;

    // password
    // get password has for the given user
    fn get_password(&self, user_id: &String) -> Result<Option<String>>;
    // set password for the given user
    fn set_password(&self, user_id: &String, password: Option<&str>) -> Result<()>;

    // // displayname
    // // get user displayname for the given user
    // fn get_displayname(&self, user_id: &String) -> Result<Option<String>>;
    // // set user displayname for the given user
    // fn set_displayname(&self, user_id: &String, displayname: Option<&str>) -> Result<()>;

    // token
    // set token-user in login
    fn set_token(&self, user_id: &String) -> Result<String>;
    // get user from token
    fn from_token(&self, token: String) -> Result<Option<String>>;
}

#[cfg(test)]
pub mod tests {

    use crate::service::services;

    // #[test]
    pub fn test_user() {
        // let tmp_dir = setup_services("test_user");
        let user = "@carl:example.com".to_string();

        // check user -> false
        assert_eq!(services().handler.exists(&user).unwrap(), false);

        // get password -> None
        assert_eq!(services().handler.get_password(&user).unwrap(), None);

        // // get displayname -> None
        // assert_eq!(services().handler.get_displayname(&user).unwrap(), None);

        // // set displayname
        // services()
        //     .handler
        //     .set_displayname(&user, Some("carl"))
        //     .unwrap();
        // // get displayname -> Some
        // assert_eq!(
        //     services().handler.get_displayname(&user).unwrap(),
        //     Some("carl".to_string())
        // );

        // set password
        services()
            .handler
            .set_password(&user, Some("password"))
            .unwrap();
        // check password
        assert!(argon2::verify_encoded(
            &services().handler.get_password(&user).unwrap().unwrap(),
            "password".as_bytes()
        )
        .unwrap());

        // set token
        let token = services().handler.set_token(&user).unwrap();
        assert_eq!(
            services().handler.from_token(token.clone()).unwrap(),
            Some(user.clone())
        );
        // set new token
        let new_token = services().handler.set_token(&user).unwrap();
        assert_eq!(
            services().handler.from_token(new_token).unwrap(),
            Some(user.clone())
        );
        // remove old token
        assert_eq!(services().handler.from_token(token.clone()).unwrap(), None);

        // check user -> true
        assert_eq!(services().handler.exists(&user).unwrap(), true);

        // get all user
        let users = services().handler.users().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0], user);
    }
}
