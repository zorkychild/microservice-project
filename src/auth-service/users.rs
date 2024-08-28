use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;
use uuid::Uuid;

use std::collections::HashMap;

pub trait Users {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String>;
    fn get_user_uuid(&self, username: String, password: String) -> Option<String>;
    fn delete_user(&mut self, user_uuid: String);
}

#[derive(Clone)]
pub struct User {
    user_uuid: String,
    username: String,
    password: String,
}

#[derive(Default)]
pub struct UsersImpl {
    uuid_to_user: HashMap<String, User>,
    username_to_user: HashMap<String, User>,
}

impl Users for UsersImpl {
    fn create_user(&mut self, username: String, password: String) -> Result<(), String> {
        // TODO: Check if username already exist. If so return an error.
        if self.username_to_user.contains_key(&username) {
            return Err(username);
        }

        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Failed to hash password.\n{e:?}"))?
            .to_string();

        let user: User = User { user_uuid: salt.to_string(), username: username, password: hashed_password }; // Create new user with unique uuid and hashed password.

        // TODO: Add user to `username_to_user` and `uuid_to_user`.
        self.uuid_to_user.insert(user.user_uuid.clone(), user.clone());
        self.username_to_user.insert(user.username.clone(), user);

        Ok(())
    }

    fn get_user_uuid(&self, username: String, password: String) -> Option<String> {
        let user: &User = self.username_to_user.get(&username)?;
        // let Some(user) = self.username_to_user.get(&username) else {
        //     return None;
        // };

        // Get user's password as `PasswordHash` instance. 
        let hashed_password = user.password.clone();
        let parsed_hash = PasswordHash::new(&hashed_password).ok()?;

        // Verify passed in password matches user's password.
        let result = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash);

        // TODO: If the username and password passed in matches the user's username and password return the user's uuid.
        if result.is_ok() {
            return Some(user.user_uuid.clone());
        }

        None
    }

    fn delete_user(&mut self, user_uuid: String) {
        // TODO: Remove user from `username_to_user` and `uuid_to_user`.
        
        if let Some(user) = self.uuid_to_user.remove(&user_uuid) {
            self.username_to_user.remove(&user.username);
            println!("Delete Successful");
            return;
        }
        // if self.uuid_to_user.contains_key(&user_uuid) {
        //     let username = &self.uuid_to_user.get(&user_uuid).unwrap().username;
        //     self.username_to_user.remove(username);
        //     self.uuid_to_user.remove(&user_uuid);
        //     println!("Delete Successful");
        //     return
        // }
        println!("Delete Unsuccessful");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert_eq!(user_service.uuid_to_user.len(), 1);
        assert_eq!(user_service.username_to_user.len(), 1);
    }

    #[test]
    fn should_fail_creating_user_with_existing_username() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let result = user_service.create_user("username".to_owned(), "password".to_owned());

        assert!(result.is_err());
    }

    #[test]
    fn should_retrieve_user_uuid() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(user_service
            .get_user_uuid("username".to_owned(), "password".to_owned())
            .is_some());
    }

    #[test]
    fn should_fail_to_retrieve_user_uuid_with_incorrect_password() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        assert!(user_service
            .get_user_uuid("username".to_owned(), "incorrect password".to_owned())
            .is_none());
    }

    #[test]
    fn should_delete_user() {
        let mut user_service = UsersImpl::default();
        user_service
            .create_user("username".to_owned(), "password".to_owned())
            .expect("should create user");

        let user_uuid = user_service
            .get_user_uuid("username".to_owned(), "password".to_owned())
            .unwrap();

        user_service.delete_user(user_uuid);

        assert_eq!(user_service.uuid_to_user.len(), 0);
        assert_eq!(user_service.username_to_user.len(), 0);
    }
}
