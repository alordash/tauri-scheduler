use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User<'a> {
    login: &'a str,
    password: &'a str,
}

impl<'a> User<'a> {
    pub fn get_login(&self) -> &'a str {
        self.login
    }

    pub fn set_login(&mut self, login: &'a str) {
        self.login = login
    }

    pub fn get_password(&self) -> &'a str {
        self.password
    }

    pub fn set_password(&mut self, password: &'a str) {
        self.password = password
    }
}
