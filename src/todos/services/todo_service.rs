use async_trait::async_trait;

use crate::core::common::error::AppError;
use crate::core::services::base_service::BaseService;
use crate::todos::dto::create_todo_dto::CreateTodoDto;
use crate::todos::entities::todo_entity::TodoEntity;
use crate::todos::repositories::todo_repository::{TodoMongoRepository, TodoRepository};

#[async_trait]
pub trait TodoService: Sync + Send {
    async fn get_todos(&self) -> Result<Vec<TodoEntity>, AppError>;
    async fn create_todo(&self, data: CreateTodoDto) -> Result<TodoEntity, AppError>;
}

impl<T> BaseService for T where T: TodoService {}

pub struct TodoServiceImpl {
    pub todo_repository: Box<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new() -> Self {
        return TodoServiceImpl {
            todo_repository: Box::new(TodoMongoRepository::new()),
        };
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_todos(&self) -> Result<Vec<TodoEntity>, AppError> {
        return self.todo_repository.find().await;
    }

    async fn create_todo(&self, data: CreateTodoDto) -> Result<TodoEntity, AppError> {
        return self
            .todo_repository
            .create(TodoEntity::new(data.title, data.deadline))
            .await;
    }
}
