use crate::auth::check_hash;
use crate::error::Error::InvalidCredential;
use crate::error::Result;
use crate::model::device::{Device, NewDevice};
use crate::model::user::{User, UserInputUnchecked};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use sqlx::PgPool;
use tracing::error;

#[derive(Clone, Debug)]
pub struct DbHandler {
    pool: PgPool,
}

impl DbHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn connect() -> Result<Self> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
            error!(error=?e, "DATABASE_URL is not set");
            std::process::exit(1);
        });

        PgPoolOptions::new()
            .connect(&url)
            .await
            .map(Self::new)
            .map_err(Into::into)
    }

    pub async fn insert_device(&self, device: NewDevice) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
INSERT INTO devices (mac, user_id, internet, date_time)
VALUES ($1, $2, $3, $4)
        "#,
            device.mac,
            device.user_id,
            device.internet,
            device.date_time
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(Into::into)
    }

    pub async fn get_devices_by_user_id(&self, id: Uuid) -> Result<Vec<Device>> {
        let records = sqlx::query!(
            r#"
                SELECT * FROM devices
                WHERE user_id=$1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|x| Device {
                id: x.id,
                mac: x.mac.to_string(),
                user_id: x.user_id,
                internet: x.internet,
                date_time: x.date_time,
            })
            .collect())
    }

    pub async fn get_device_by_mac(&self, mac: String) -> Result<Option<Device>> {
        Ok(sqlx::query!(
            r#"
                SELECT * FROM devices
                WHERE mac=$1
            "#,
            mac
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|x| Device {
            id: x.id,
            mac: x.mac.to_string(),
            user_id: x.user_id,
            internet: x.internet,
            date_time: x.date_time,
        }))
    }

    pub async fn update_device(&self, device: Device) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE devices
            SET mac = $1,
            user_id = $2,
            internet = $3,
            date_time = $4
            WHERE id=$5
        "#,
            device.mac,
            device.user_id,
            device.internet,
            device.date_time,
            device.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(Into::into)
    }

    pub async fn delete_device(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            DELETE FROM devices
            WHERE id=$1
        "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(Into::into)
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let mut res: Vec<Device> = Vec::new();

        let q = sqlx::query!(
            r#"
                SELECT * FROM devices
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        for x in &q {
            res.push(Device {
                id: x.id,
                mac: x.mac.to_string(),
                user_id: x.user_id,
                internet: x.internet,
                date_time: x.date_time,
            });
        }

        Ok(res)
    }

    pub async fn insert_user(&self, user: UserInputUnchecked) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
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
            "user",
            false
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(Into::into)
    }

    pub async fn update_user(&self, user: User) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
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
            user.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(Into::into)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id=$1
        "#,
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(Into::into)
    }

    pub async fn check_password(&self, login: String, password: String) -> Result<(String, Uuid)> {
        let res = sqlx::query!(
            r#"
                SELECT password, role, id FROM users
                WHERE username=$1
            "#,
            login
        )
        .fetch_optional(&self.pool)
        .await?;
        match res {
            Some(x) => {
                if check_hash(password, x.password.to_string()) {
                    Ok((x.role.to_string(), x.id))
                } else {
                    Err(InvalidCredential)
                }
            }
            None => Err(InvalidCredential),
        }
    }

    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>> {
        let res = sqlx::query!(
            r#"
                SELECT * FROM users
                WHERE id=$1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(match res {
            Some(x) => Some(User {
                id: x.id,
                username: x.username.to_string(),
                firstname: x.firstname.to_string(),
                lastname: x.lastname.to_string(),
                email: x.email.to_string(),
                password: x.password.to_string(),
                phone: x.phone.to_string(),
                role: x.role.to_string(),
                is_allowed: x.is_allowed,
            }),

            None => None,
        })
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let mut res: Vec<User> = Vec::new();

        let q = sqlx::query!(
            r#"
                SELECT * FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        for x in &q {
            res.push(User {
                id: x.id,
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

        Ok(res)
    }
}
