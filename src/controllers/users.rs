use crate::error::Result;
use crate::models::user::{ImgUrl, NewUser, ProfImg, User, UserConditions, UserId, UserList};
use crate::repositories::RepoExt;
use crate::usecases;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};

pub async fn index(
    Query(conditions): Query<UserConditions>,
    Extension(repo): RepoExt,
) -> Result<Json<UserList>> {
    let users = usecases::users::search(repo.clone(), &conditions).await?;
    Ok(Json(users))
}

pub async fn view(Path(user_id): Path<i32>, Extension(repo): RepoExt) -> Result<Json<User>> {
    let user = usecases::users::view(repo.clone(), user_id).await?;
    Ok(Json(user))
}

pub async fn add(Json(new_user): Json<NewUser>, Extension(repo): RepoExt) -> Result<Json<UserId>> {
    let user_id = usecases::users::add(repo.clone(), &new_user).await?;
    Ok(Json(user_id))
}

pub async fn edit_prof_img(
    Json(prof_img): Json<ProfImg>,
    Extension(repo): RepoExt,
) -> Result<Json<ImgUrl>> {
    let img_url = usecases::users::edit_prof_img(repo.clone(), &prof_img).await?;
    Ok(Json(img_url))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
