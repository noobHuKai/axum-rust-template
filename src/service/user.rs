use crate::error::AppError;
use crate::model::database::User;
use crate::model::CreateUserReq;
use sqlx::PgPool;

pub async fn query_by_username_and_password(
    conn: &PgPool,
    username: String,
    password: String,
) -> sqlx::Result<User> {
    let user =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 AND password = $2")
            .bind(username)
            .bind(password)
            .fetch_one(conn)
            .await?;
    Ok(user)
}

pub async fn query_all_user(conn: &PgPool) -> sqlx::Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(conn)
        .await?;
    Ok(users)
}

pub async fn insert_user(conn: &PgPool, req: &CreateUserReq) -> sqlx::Result<()> {
    sqlx::query(
        "insert into users (username, password, create_at, update_at, role)
    values ($1, $2 , now(), now(), $3)",
    )
    .bind(&req.username)
    .bind(&req.password)
    .bind(&req.role)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn delete_user(conn: &PgPool, uid: String) -> Result<(), AppError> {
    let uid = uuid::Uuid::parse_str(&uid)?;
    sqlx::query("delete from users where id = $1")
        .bind(uid)
        .execute(conn)
        .await?;
    Ok(())
}
