use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email, user);
            Ok(())
        }
    }
}

// TODO: Implement a public method called `get_user`, which takes an
// immutable reference to self and an email string slice as arguments.
// This function should return a `Result` type containing either a
// `User` object or a `UserStoreError`.
// Return `UserStoreError::UserNotFound` if the user can not be found.
impl HashmapUserStore {
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }
}
// TODO: Implement a public method called `validate_user`, which takes an
// immutable reference to self, an email string slice, and a password string slice
// as arguments. `validate_user` should return a `Result` type containing either a
// unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
// Return `UserStoreError::UserNotFound` if the user can not be found.
// Return `UserStoreError::InvalidCredentials` if the password is incorrect.

// TODO: Add unit tests for your `HashmapUserStore` implementation
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     fn test_add_user() {
//         todo!()
//     }

//     #[tokio::test]
//     fn test_get_user() {
//         todo!()
//     }

//     #[tokio::test]
//     fn test_validate_user() {
//         todo!()
//     }
// }
