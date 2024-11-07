use std::str::FromStr;

use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tracing::error;
use crate::auth::check_hash;
use crate::models::{Session, User};

#[derive(Clone, Debug)]
pub struct DbHandler {
    pool: PgPool,
}

impl DbHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn connect() -> Option<Self> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
            error!(error=?e, "DATABASE_URL is not set");
            std::process::exit(1);
        });

        match PgPoolOptions::new().connect(&url).await {
            Ok(p) => return Some(Self::new(p)),
            Err(_) => return None,
        }
    }

    pub async fn insert_session(&self, session: Session) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        let user_id = match session.user_id {
            Some(i) => match Uuid::from_str(i.as_str()) {
                Ok(u) => Some(u),
                Err(_) => None,
            },
            None => None,
        };

        match sqlx::query!(
            r#"
INSERT INTO sessions (ip4, user_id, internet, date_time)
VALUES ($1, $2, $3, $4)
        "#,
            session.ip4,
            user_id,
            session.internet,
            session.date_time
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn get_session_by_user_id(&self, id: String) -> Option<Session> {
        return match sqlx::query!(
            r#"
                SELECT * FROM sessions
                WHERE user_id=$1
            "#,
            match Uuid::from_str(id.as_str()) {
                Ok(u) => Some(u),
                Err(_) => None,
            }
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(x) => Some(Session {
                id: Some(x.id.as_hyphenated().to_string()),
                ip4: x.ip4.to_string(),
                user_id: match x.user_id {
                    Some(i) => Some(i.as_hyphenated().to_string()),
                    None => None,
                },
                internet: x.internet,
                date_time: x.date_time,
            }),

            Err(_) => None,
        };
    }

    pub async fn update_session(&self, session: Session) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        match sqlx::query!(
            r#"
            UPDATE sessions
            SET ip4 = $1,
            user_id = $2,
            internet = $3,
            date_time = $4
            WHERE id=$5
        "#,
            session.ip4,
            match Uuid::from_str(
                session
                    .user_id
                    .expect("[ASSERTION] could not get id")
                    .as_str()
            ) {
                Ok(u) => Some(u),
                Err(_) => return false,
            },
            session.internet,
            session.date_time,
            match Uuid::from_str(session.id.expect("[ASSERTION] could not get id").as_str()) {
                Ok(u) => Some(u),
                Err(_) => return false,
            }
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn delete_session(&self, id: String) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        match sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE id=$1
        "#,
            match Uuid::from_str(id.as_str()) {
                Ok(u) => Some(u),
                Err(_) => return false,
            }
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn get_sessions(&self) -> Option<Vec<Session>> {
        let mut res: Vec<Session> = Vec::new();

        match sqlx::query!(
            r#"
                SELECT * FROM sessions
            "#
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(q) => {
                for x in &q {
                    let user_id = match x.user_id {
                        Some(i) => Some(i.as_hyphenated().to_string()),
                        None => None,
                    };
                    res.push(Session {
                        id: Some(x.id.as_hyphenated().to_string()),
                        ip4: x.ip4.to_string(),
                        user_id,
                        internet: x.internet,
                        date_time: x.date_time,
                    });
                }
            }
            Err(_) => return None,
        };

        Some(res)
    }

    pub async fn insert_user(&self, user: User) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        match sqlx::query!(
            r#"
INSERT INTO users (username, firstname, lastname, email, password, phone, role, is_allowed)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
            user.username,
            user.firstname,
            user.lastname,
            user.email,
            user.password,
            user.phone,
            user.role,
            user.is_allowed
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn update_user(&self, user: User) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        match sqlx::query!(
            r#"
            UPDATE users
            SET username = $1,
            firstname = $2,
            lastname = $3,
            email = $4,
            password = $5,
            phone = $6,
            role = $7 ,
            is_allowed = $8
            WHERE id=$9
        "#,
            user.username,
            user.firstname,
            user.lastname,
            user.email,
            user.password,
            user.phone,
            user.role,
            user.is_allowed,
            match Uuid::from_str(user.id.expect("[ASSERTION] could not get id").as_str()) {
                Ok(u) => Some(u),
                Err(_) => return false,
            }
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn delete_user(&self, id: String) -> bool {
        let mut tx = match self.pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return false;
            }
        };

        match sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id=$1
        "#,
            match Uuid::from_str(id.as_str()) {
                Ok(u) => Some(u),
                Err(_) => return false,
            }
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => return false,
        };

        return match tx.commit().await {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub async fn check_password(
        &self,
        login: String,
        password: String,
    ) -> Option<(String, String)> {
        return match sqlx::query!(
            r#"
                SELECT password, role, id FROM users
                WHERE username=$1
            "#,
            login
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(x) => {
                if check_hash(password, x.password.to_string()) {
                    Some((x.role.to_string(), x.id.as_hyphenated().to_string()))
                } else {
                    None
                }
            }
            Err(_) => None,
        };
    }

    pub async fn get_user(&self, id: String) -> Option<User> {
        return match sqlx::query!(
            r#"
                SELECT * FROM users
                WHERE id=$1
            "#,
            match Uuid::from_str(id.as_str()) {
                Ok(u) => Some(u),
                Err(_) => None,
            }
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(x) => Some(User {
                id: Some(x.id.as_hyphenated().to_string()),
                username: x.username.to_string(),
                firstname: x.firstname.to_string(),
                lastname: x.lastname.to_string(),
                email: x.email.to_string(),
                password: x.password.to_string(),
                phone: x.phone.to_string(),
                role: x.role.to_string(),
                is_allowed: x.is_allowed,
            }),

            Err(_) => None,
        };
    }

    pub async fn get_users(&self) -> Option<Vec<User>> {
        let mut res: Vec<User> = Vec::new();

        match sqlx::query!(
            r#"
                SELECT * FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await
        {
            Ok(q) => {
                for x in &q {
                    res.push(User {
                        id: Some(x.id.as_hyphenated().to_string()),
                        username: x.username.to_string(),
                        firstname: x.firstname.to_string(),
                        lastname: x.lastname.to_string(),
                        email: x.email.to_string(),
                        password: x.password.to_string(),
                        phone: x.phone.to_string(),
                        role: x.role.to_string(),
                        is_allowed: x.is_allowed,
                    });
                }
            }
            Err(_) => return None,
        };

        Some(res)
    }
}
