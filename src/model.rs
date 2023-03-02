use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User<'a> {
    pub full_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,

    #[serde(skip_serializing)]
    password: &'a str,
}

impl<'a> User<'a> {
    pub fn new(username: &'a str, password: &'a str, email: &'a str, full_name: &'a str) -> User<'a> {
        User {
            full_name,
            username,
            email,
            password,
        }
    }

    pub fn get_password(&self) -> &str {
        self.password
    }

    pub fn set_password(&mut self, password: &'a str) {
        self.password = password;
    }
}

impl<'a> Display for User<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name)
    }
}
