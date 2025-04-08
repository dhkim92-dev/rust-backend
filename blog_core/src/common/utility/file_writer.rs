use std::sync::Arc;
use std::fs::create_dir_all;
use image::DynamicImage;
use shaku::{Interface,Component};
use uuid::Uuid;
use crate::common::error::error_code::ErrorCode;
use crate::{common::AppError, config::ConfigProvider};

pub struct FileUploadResult {
    pub storage_path: String,
    pub access_path: String
}

#[async_trait::async_trait]
pub trait FileWriter: Interface {

    async fn write_image(&self, image: DynamicImage) -> Result<FileUploadResult, AppError> ;
}

#[derive(Component)]
#[shaku(interface = FileWriter)]
pub struct FileWriterImpl {
    #[shaku(inject)]
    config_provider: Arc<dyn ConfigProvider>,
}

#[async_trait::async_trait]
impl FileWriter for FileWriterImpl {

    async fn write_image(&self, image: DynamicImage) -> Result<FileUploadResult, AppError> {
        let file_name = self.generate_file_name();
        let parent_path = self.generate_parent_path("images", &file_name);
        self.create_parent_path(&parent_path).await?;
        let ext = "png";
        let full_path = self.composite(file_name, parent_path, ext.to_owned());

        let result = FileUploadResult {
            storage_path: format!("{}{}", self.config_provider.get().storage_path, full_path),
            access_path: format!("{}{}", self.config_provider.get().media_url, full_path)
        };

        image.save_with_format(result.storage_path.clone(), image::ImageFormat::Png).unwrap();
        Ok(result)
    }
}

impl FileWriterImpl {

    fn generate_file_name(&self) -> Uuid {
        uuid::Uuid::new_v4()
    }

    fn generate_parent_path(&self, type_dir: &str, name: &Uuid) -> String {
        let name_string = name.to_string();
        let first_ch = name_string.chars().nth(0).unwrap();
        let second_ch = name_string.chars().nth(1).unwrap();
        let mut path = "/".to_string();
        path.push_str(type_dir);
        path.push_str("/");
        path.push_str(first_ch.to_string().as_str());
        path.push_str("/");
        path.push_str(second_ch.to_string().as_str());
        path
    }

    async fn create_parent_path(&self, path: &String) -> Result<(), AppError> {
        let media_origin = self.config_provider.get().storage_path.clone();
        let mut full_path = media_origin;
        full_path.push_str(path.as_str());

        match create_dir_all(full_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("디렉토리 생성 실패 : {}", e);
                Err(AppError::with_message(ErrorCode::InternalServerError, "저장소 경로 생성 실패"))
            }
        }
    }

    fn composite(&self, fname: Uuid, parent_path: String, ext: String) -> String {
        let mut path = parent_path;
        path.push_str("/");
        path.push_str(fname.to_string().as_str());
        path.push_str(".");
        path.push_str(ext.as_str());
        path
    }
}
