use crate::models::auth::LoginRequest;
use crate::models::user::User;
use crate::db::get_db;
use anyhow::Result;
use bcrypt::verify;

pub async fn login(login: LoginRequest) -> Result<User> {
    let pool = get_db().await?;
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password, role FROM users WHERE email = $1",
        login.email
    )
    .fetch_one(&pool)
    .await?;

    let verified = verify(&login.password, &user.password)?;
    dbg!(verified, &login.password, &user.password);
    if !verified {
        return Err(anyhow::anyhow!("Invalid password"));
    }

    Ok(user)
} 