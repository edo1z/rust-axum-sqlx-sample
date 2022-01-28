use crate::error::Result;
use crate::models::user::{ImgUrl, NewUser, ProfileImage, User, UserConditions, UserId, UserList};
use crate::repositories::{user::UserRepo, RepoExt, Repositories};
use crate::usecases::users::search;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};

pub async fn index(
    Query(conditions): Query<UserConditions>,
    Extension(repo): RepoExt,
) -> Result<Json<UserList>> {
    let users = search(repo.clone(), &conditions).await?;
    Ok(Json(users))
}

pub async fn view(Path(user_id): Path<i32>, Extension(repo): RepoExt) -> Result<Json<User>> {
    let user = repo.user().find_by_id(user_id).await?;
    Ok(Json(user))
}

pub async fn add(Json(user_data): Json<NewUser>, Extension(repo): RepoExt) -> Result<Json<UserId>> {
    let user_id = repo.user().add(&user_data).await?;
    Ok(Json(user_id))
}

pub async fn edit_profile_img(
    Json(_profile_image): Json<ProfileImage>,
    Extension(_repo): RepoExt,
) -> Result<Json<ImgUrl>> {
    Ok(Json(ImgUrl {
        url: String::from("https://example.com/hoge.png"),
    }))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
