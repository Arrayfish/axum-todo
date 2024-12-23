// 一旦DBアクセスも同じところでいいや
use ::entity::{todo, todo::Entity as Todo};

// Query
pub async fn get_user_todos(user_pid: Uuid) -> anyhow::Result<Vec<todo::Model>> {
    let todo = Todo::find()
        .filter(todo::Column::UserPid.eq(user_pid))
        .all().await?;
    Ok(todo)
}

// Command
pub async fn create_todo(
    user_pid: Uuid,
    content: String,
) -> anyhow::Result<todo::Model> {
    let todo = Todo::insert()
        .set(todo::Column::UserPid, user_pid)
        .set(todo::Column::Content, content)
        .exec().await?;
    Ok(todo)
}