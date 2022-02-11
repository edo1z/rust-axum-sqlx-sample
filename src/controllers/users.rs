use crate::error::{Result, AppError};
use crate::models::user::{ImgUrl, NewUser, ProfImg, User, UserConditions, UserId, UserList};
use crate::repositories::RepoExt;
use crate::usecases;
use axum::{
    extract::{Extension, Path, Query, Multipart},
    http::StatusCode,
    Json,
};

use image::{ImageFormat, guess_format};
use std::io::Cursor;
use std::fs::File;
use exif::{In, Reader, Tag};

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
    mut multipart: Multipart,
    Extension(repo): RepoExt,
) -> Result<Json<User>> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("{name}: {:?}", data);
    }

    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let bytes:Vec<u8> = data.into_iter().collect();
        let (format, ext) = match guess_format(&bytes) {
            Ok(ImageFormat::Png) => Ok((ImageFormat::Png, "png")),
            Ok(ImageFormat::Jpeg) => Ok((ImageFormat::Jpeg, "jpg")),
            Ok(ImageFormat::Gif) => Ok((ImageFormat::Gif, "gif")),
            _ => Err(AppError::InvalidFileFormat),
        }?;
        let mut buf = Cursor::new(&bytes);
        let mut orientation = 1;
        if let Ok(exif) = Reader::new().read_from_container(&mut buf) {
            if let Some(o) = exif.get_field(Tag::Orientation, In::PRIMARY) {
                if let Some(v @ 1..=8) = o.value.get_uint(0) {
                    orientation = v;
                }
            }
        }
        println!("{orientation}");
        match image::load_from_memory_with_format(&bytes, format) {
            Ok(img) => {
                let new_img = img.thumbnail(300, 300).blur(2.0);
                let mut output = File::create(format!("new_img.{ext}")).unwrap();
                new_img.write_to(&mut output, format).unwrap();
            }
            Err(_) => {
                println!("Invalid File");
            }
        }
    }
    let user = usecases::users::view(repo.clone(), 10).await?;
    Ok(Json(user))
}

pub async fn edit() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
