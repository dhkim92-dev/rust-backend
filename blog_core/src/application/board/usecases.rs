
struct CreateCategoryCommand {
    pub name: String,
}

struct ModifyCategoryCommand {
    pub name: String,
}

#[async_trait::async_trait]
pub trait CategoryCreateUsecase {
    fn create_category(&self, command: CreateCategoryCommand) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait CategoryModifyUsecase {
    fn modify_category(&self, id: u64, command: ModifyCategoryCommand) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait CategoryDeleteUsecase {
    fn delete_category(&self, id: u64) -> Result<(), String>;
}

