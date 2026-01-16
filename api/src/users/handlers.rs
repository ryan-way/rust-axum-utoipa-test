use super::models::*;

use axum::extract::{Json, State, Path};
use sqlx::{QueryBuilder, Sqlite};
use sqlx::sqlite::SqlitePool;
use jsonapi::{self, model::vec_to_jsonapi_document};

use jsonapi::model::*;

#[utoipa::path(get, path="/users", responses((status = OK, body = JsonApiResponse<Vec<User>>)))]
pub async fn get_users(State(pool): State<SqlitePool>) -> Json<JsonApiDocument> {
    let users: Vec<User> = sqlx::query_as!(User, r#"
    select id, username from users
    "#).fetch_all(&pool).await.unwrap();

    Json(vec_to_jsonapi_document(users))
}


#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, body = JsonApiResponse<User>)
    )
)]
pub async fn get_user(State(pool): State<SqlitePool>, Path(user_id): Path<i64>) -> Json<JsonApiDocument> {
    let user: User = sqlx::query_as!(User, r#"select id, username from users where id = ?"#, user_id)
        .fetch_one(&pool)
        .await
        .unwrap();


    Json(user.to_jsonapi_document())
}


#[utoipa::path(post, path="/users", responses((status = OK, body = JsonApiResponse<User>)))]
pub async fn post_user(State(pool): State<SqlitePool>, Json(new_user): Json<NewUser>) -> Json<JsonApiDocument> {
    let user: User = sqlx::query_as!(User, r#"insert into users (username) values (?) returning id, username"#, new_user.username).fetch_one(&pool).await.unwrap();
    Json(user.to_jsonapi_document())
}



#[utoipa::path(patch, path="/users/{id}", responses((status = OK, body = JsonApiResponse<User>)))]
pub async fn patch_user(State(pool): State<SqlitePool>, Path(user_id): Path<i64>, Json(patch_user): Json<PatchUser>) -> Json<JsonApiDocument> {
    let mut qb = QueryBuilder::<Sqlite>::new("update users set ");

    let mut separated = qb.separated(",");

    if let Some(username) = patch_user.username {
        separated.push_unseparated(" username = ").push_bind(username);
    }

    qb.push(" where id = ").push_bind(user_id);
    qb.push(" returning id, username");

    dbg!(qb.sql());

    let user: User = qb.build_query_as::<User>().fetch_one(&pool).await.unwrap();

    Json(user.to_jsonapi_document())
}

#[utoipa::path(delete, path="/users/{id}", responses((status = OK, body = JsonApiResponse<User>)))]
pub async fn delete_user(Path(id): Path<i64>) -> Json<User> {
    Json(User { id, username: "Ryan Way".to_owned()})
}

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