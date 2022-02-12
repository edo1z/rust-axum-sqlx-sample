use crate::error::{AppError, Result};
use crate::models::user::{NewUser, User, UserConditions, UserId, UserList};
use crate::repositories::RepoExt;
use crate::services::img_upload::img_upload;
use crate::usecases;
use anyhow::anyhow;
use axum::{
    extract::{ContentLengthLimit, Extension, Multipart, Path, Query},
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
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, { 5 * 1024 * 1024 }>,
    Extension(repo): RepoExt,
) -> Result<Json<User>> {
    let result = _multipart_for_edit_prof_img(multipart).await;
    if let Err(e) = result {
        return Err(AppError::MultipartError(e.to_string()));
    }
    let (user_id, prof_img) = result.unwrap();
    let result = img_upload(prof_img);
    println!("user_id: {user_id} upload result {result:?}");
    let user = usecases::users::view(repo.clone(), 10).await?;
    Ok(Json(user))
}

async fn _multipart_for_edit_prof_img(mut multipart: Multipart) -> anyhow::Result<(i32, Vec<u8>)> {
    let mut user_id: Option<i32> = None;
    let mut prof_img: Vec<u8> = vec![];
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        let bytes: Vec<u8> = field.bytes().await?.into_iter().collect();
        match &*name {
            "user_id" => user_id = Some(std::str::from_utf8(&bytes)?.parse()?),
            "prof_img" => prof_img = bytes,
            _ => return Err(anyhow!("Invalid Parameter")),
        }
    }
    Ok((user_id.unwrap(), prof_img))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
