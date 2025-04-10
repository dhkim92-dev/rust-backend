use std::sync::Arc;
use axum::{extract::{State, Multipart}, Extension};
use serde::Serialize;
use shaku::HasComponent;
use crate::{common::{error_code::ErrorCode, file_writer::FileWriter, AppError, LoginMember, ReturnValue}, di::AppContext};

pub async fn upload_image(
    State(ctx): State<Arc<AppContext>>,
    Extension(login_member): Extension<LoginMember>,
    mut form: Multipart
) -> Result<ReturnValue<ImageUploadResposne>, AppError> {
    // let cfg_provider: &dyn ConfigProvider = ctx.resolve_ref();
    let file_writer: &dyn FileWriter = ctx.resolve_ref();
    let field = form.next_field().await.unwrap().unwrap();
    let name = field.name().unwrap().to_string();

    if !login_member.is_admin() {
        return Err(AppError::from(ErrorCode::Forbidden));
    }

    if name != "file" {
        return Err(AppError::from(ErrorCode::BadRequest));
    }
    let content_type = field.content_type().unwrap().to_string();

    if !content_type.starts_with("image/") {
        return Err(AppError::from(ErrorCode::BadRequest));
    }

    let data = field.bytes().await.unwrap();
    let mut img = image::load_from_memory(&data).unwrap();
    let width = img.width();
    let height = img.height();

    if width > 1280 {
        let ratio: f32 = (height as f32)/(width as f32);
        let new_height = (1280.0 * ratio) as u32;
        img = img.resize(1280, new_height, image::imageops::FilterType::Lanczos3);
    }

    let ret = file_writer.write_image(img).await?;

    Ok(ReturnValue { 
        status: 200,
        message: "이미지가 업로드 되었습니다.".to_owned(), 
        data:  ImageUploadResposne {
            width: width as u16,
            height: height as u16,
            format: "png".to_string(),
            url: ret.access_path,
            storage_path: ret.storage_path
        }
    })
}


#[derive(Serialize)]
pub struct ImageUploadResposne {
    pub width: u16,
    pub height: u16,
    pub format: String,
    pub url: String,
    pub storage_path: String
}
