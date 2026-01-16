
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use jsonapi::jsonapi_model;
use jsonapi::model::*;


// Mapper struct since JsonApiDocument does not implement ToSchema for utoipa, and this cannot be done in this crate
#[derive(ToSchema)]
pub struct JsonApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
}
jsonapi_model!(User; "user");


#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewUser {
    pub username: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema)]
pub struct PatchUser {
    pub username: Option<String>,
}

// Example of establishing a relationship and includes field using the jsonapi_model macro
// #[derive(Debug, Serialize, Deserialize, FromRow)]
// pub struct UserDto {
//     id: i64,
//     username: String,
//     related_info: RelatedInfo,
// }
// jsonapi_model!(UserDto; "user_dto"; has one related_info);

// impl From<User> for UserDto {
//     fn from(value: User) -> Self {
//         UserDto {
//             id: value.id,
//             username: value.username,
//             related_info: RelatedInfo { id: 1, num: 10 }
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct RelatedInfo {
//     id: i64,
//     num: i32,
// }
// jsonapi_model!(RelatedInfo; "related_info");