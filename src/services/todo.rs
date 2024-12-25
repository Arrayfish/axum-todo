// 一旦DBアクセスも同じところでいいや
use ::entity::{todo, todo::Entity as Todo};
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::{Set, NotSet, Unchanged};
use uuid::Uuid;

// Query
pub async fn get_user_todos(
    db: &Db,
    user_id: Uuid
) -> anyhow::Result<Vec<todo::Model>> {
    let todo_list = Todo::find_by_id(user_id)
        .all(db).await?;
    Ok(todo_list)
}

// Command
pub async fn create_todo(
    user_id: Uuid,
    content: String,
) -> anyhow::Result<todo::Model> {
    let todo = todo::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        content: Set(content),
        done: Set(false),
        ..Default::default()
    };
    let todo: todo::Model = todo.insert(db).await?;
    Ok(todo)
}

pub async fn update_todo(
    todo_id: Uuid,
    content: Option<String>,
    done: Option<bool>,
) -> anyhow::Result<todo::Model> {
    let todo: Option<todo::Model> = Todo::find_by_id(todo_id)
        .one(db).await?;
    let mut todo: todo::ActiveModel = todo.unwrap().into();
    match content {
        Some(content) => todo.content = Set(content),
        None => (),
    }
    match done {
        Some(done) => todo.done = Set(done),
        None => (),
    }

    let todo: todo::Model = todo.update(db).await?;
    Ok(todo)
}

pub async fn delete_todo(
    todo_id: Uuid,
) -> anyhow::Result<()> {
    let todo: Option<todo::Model> = Todo::find_by_id(todo_id)
        .one(db).await?;
    todo.unwrap().delete(db).await?;
    Ok(())
}