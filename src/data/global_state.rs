use leptos::RwSignal;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
pub struct UserBuilder {
    pub user_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub id: Option<i32>,
    pub role: Option<String>,
}

impl UserBuilder {
    pub fn to_user(self) -> Result<User, String> {
        match self {
            UserBuilder {
                user_name: Some(user_name),
                first_name: Some(first_name),
                last_name: Some(last_name),
                id: Some(id),
                role: Some(role),
            } => Ok(User {
                user_name,
                first_name,
                last_name,
                id,
                role,
            }),
            UserBuilder {
                user_name,
                first_name,
                last_name,
                id,
                role,
            } => {
                let error_message = format!(
                    "Couldn't create User. The following fields weren't found: {} {} {} {} {}",
                    user_name.map_or("user_name", |_| ""),
                    first_name.map_or("first_name", |_| ""),
                    last_name.map_or("last_name", |_| ""),
                    id.map_or("id", |_| ""),
                    role.map_or("role", |_| ""),
                );
                Err(error_message)
            }
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub id: i32,
    // TODO make role an enum
    pub role: String,
}

impl User {
    pub fn fields() -> core::array::IntoIter<UserFields, 5> {
        let fields = [
            UserFields::UserName,
            UserFields::FirstName,
            UserFields::LastName,
            UserFields::Id,
            UserFields::Role,
        ];
        fields.into_iter()
    }
}

pub enum UserFields {
    UserName,
    FirstName,
    LastName,
    Id,
    Role,
}

impl UserFields {
    pub fn key(&self) -> &str {
        match self {
            UserFields::UserName => "user_name",
            UserFields::FirstName => "first_name",
            UserFields::LastName => "last_name",
            UserFields::Id => "id",
            UserFields::Role => "role",
        }
    }
}

#[derive(Clone)]
pub enum Authentication {
    Authenticated(User),
    Unauthenticated,
}

impl Authentication {
    pub fn is_authenticated(&self) -> bool {
        match self {
            Authentication::Authenticated(_) => true,
            Authentication::Unauthenticated => false,
        }
    }
    pub fn is_unauthenticated(&self) -> bool {
        !self.is_authenticated()
    }
    pub fn get_user(&self) -> Option<&User> {
        match self {
            Authentication::Authenticated(user) => Some(user),
            Authentication::Unauthenticated => None,
        }
    }
}

pub type AuthContext = RwSignal<Authentication>;
