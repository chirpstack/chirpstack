use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Pbkdf2,
};
use rand_core::OsRng;
use tokio::task;
use tracing::info;
use uuid::Uuid;
use validator::validate_email;

use super::error::Error;
use super::get_db_conn;
use super::schema::user;

#[derive(Queryable, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = user)]
pub struct User {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_admin: bool,
    pub is_active: bool,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub note: String,
}

impl Default for User {
    fn default() -> Self {
        let now = Utc::now();

        User {
            id: Uuid::new_v4(),
            external_id: None,
            created_at: now,
            updated_at: now,
            is_admin: false,
            is_active: false,
            email: "".into(),
            email_verified: false,
            password_hash: "".into(),
            note: "".into(),
        }
    }
}

impl User {
    pub fn validate(&self) -> Result<(), Error> {
        if self.email != "admin" && !validate_email(&self.email) {
            return Err(Error::InvalidEmail);
        }

        Ok(())
    }

    pub fn set_password_hash(&mut self, pw: &str, rounds: u32) -> Result<(), Error> {
        self.password_hash = hash_password(pw, rounds)?;
        Ok(())
    }
}

pub async fn create(u: User) -> Result<User, Error> {
    u.validate()?;
    let u = task::spawn_blocking({
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;

            diesel::insert_into(user::table)
                .values(&u)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, u.id.to_string()))
        }
    })
    .await??;
    info!(id = %u.id, "User created");
    Ok(u)
}

pub async fn get(id: &Uuid) -> Result<User, Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;
            let u = user::dsl::user
                .find(&id)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;
            Ok(u)
        }
    })
    .await?
}

pub async fn get_by_email(email: &str) -> Result<User, Error> {
    task::spawn_blocking({
        let email = email.to_string();
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;
            let u = user::dsl::user
                .filter(user::dsl::email.eq(&email))
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, email))?;
            Ok(u)
        }
    })
    .await?
}

pub async fn get_by_external_id(external_id: &str) -> Result<User, Error> {
    task::spawn_blocking({
        let external_id = external_id.to_string();
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;
            let u = user::dsl::user
                .filter(user::dsl::external_id.eq(&external_id))
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, external_id))?;
            Ok(u)
        }
    })
    .await?
}

pub async fn get_by_email_and_pw(email: &str, pw: &str) -> Result<User, Error> {
    task::spawn_blocking({
        let email = email.to_string();
        let pw = pw.to_string();
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;
            let u: User = match user::dsl::user
                .filter(user::dsl::email.eq(&email))
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, email))
            {
                Ok(v) => v,
                Err(Error::NotFound(_)) => {
                    return Err(Error::InvalidUsernameOrPassword);
                }
                Err(v) => {
                    return Err(v);
                }
            };

            if verify_password(&pw, &u.password_hash) {
                return Ok(u);
            }

            Err(Error::InvalidUsernameOrPassword)
        }
    })
    .await?
}

pub async fn update(u: User) -> Result<User, Error> {
    u.validate()?;
    let u = task::spawn_blocking({
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;
            diesel::update(user::dsl::user.find(&u.id))
                .set((
                    user::updated_at.eq(Utc::now()),
                    user::is_admin.eq(&u.is_admin),
                    user::is_active.eq(&u.is_active),
                    user::email.eq(&u.email),
                    user::email_verified.eq(&u.email_verified),
                    user::note.eq(&u.note),
                    user::external_id.eq(&u.external_id),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, u.id.to_string()))
        }
    })
    .await??;
    info!(user_id = %u.id, "User updated");
    Ok(u)
}

pub async fn set_password_hash(id: &Uuid, hash: &str) -> Result<User, Error> {
    let u = task::spawn_blocking({
        let id = *id;
        let hash = hash.to_string();
        move || -> Result<User, Error> {
            let mut c = get_db_conn()?;
            diesel::update(user::dsl::user.find(&id))
                .set(user::password_hash.eq(&hash))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))
        }
    })
    .await??;
    info!(id = %id, "Password set");
    Ok(u)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(user::dsl::user.find(&id))
                .execute(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;

            if ra == 0 {
                return Err(Error::NotFound(id.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(user_id = %id, "User deleted");
    Ok(())
}

pub async fn get_count() -> Result<i64, Error> {
    task::spawn_blocking({
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let count = user::dsl::user.select(dsl::count_star()).first(&mut c)?;
            Ok(count)
        }
    })
    .await?
}

pub async fn list(limit: i64, offset: i64) -> Result<Vec<User>, Error> {
    task::spawn_blocking({
        move || -> Result<Vec<User>, Error> {
            let mut c = get_db_conn()?;
            let items = user::dsl::user
                .order_by(user::dsl::email)
                .limit(limit)
                .offset(offset)
                .load(&mut c)?;
            Ok(items)
        }
    })
    .await?
}

// The output format is documented here:
// https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#specification
fn hash_password(pw: &str, rounds: u32) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash_resp = Pbkdf2.hash_password_customized(
        pw.as_bytes(),
        Some(Algorithm::Pbkdf2Sha512.ident()),
        None,
        pbkdf2::Params {
            rounds,
            ..Default::default()
        },
        salt.as_salt(),
    );

    match hash_resp {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(Error::HashPassword(format!("{}", e))),
    }
}

fn verify_password(pw: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(v) => v,
        Err(_) => {
            return false;
        }
    };

    Pbkdf2.verify_password(pw.as_bytes(), &parsed).is_ok()
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

    pub async fn create_user() -> User {
        let mut user = User {
            is_admin: true,
            is_active: true,
            email: "test@example.com".into(),
            email_verified: true,
            ..Default::default()
        };
        user.set_password_hash(&"password!", 1).unwrap();
        create(user).await.unwrap()
    }

    #[test]
    fn test_hash_password() {
        assert_eq!(true, hash_password(&"foobar", 1000).is_ok());
    }

    #[test]
    fn test_verify_password() {
        // this is the ChirpStack Application Server default admin hash, with == removed
        // to test the compatibility betweeh the two pbkdf2 implementations.
        assert_eq!(true, verify_password(&"admin", &"$pbkdf2-sha512$i=1,l=64$l8zGKtxRESq3PA2kFhHRWA$H3lGMxOt55wjwoc+myeOoABofJY9oDpldJa7fhqdjbh700V6FLPML75UmBOt9J5VFNjAL1AvqCozA1HJM0QVGA"));
    }

    #[tokio::test]
    async fn test_user() {
        let _guard = test::prepare().await;
        let mut user = create_user().await;

        // get
        let user_get = get(&user.id).await.unwrap();
        assert_eq!(user, user_get);

        // update
        user.external_id = Some("external_id".into());
        user = update(user).await.unwrap();

        // get by external id
        let user_get = get_by_external_id(&"external_id").await.unwrap();
        assert_eq!(user, user_get);

        // get_by_email_and_pw
        assert_eq!(
            true,
            get_by_email_and_pw(&"test@example.com", &"bar")
                .await
                .is_err()
        );
        let user_get = get_by_email_and_pw(&"test@example.com", &"password!")
            .await
            .unwrap();
        assert_eq!(user, user_get);

        // delete
        delete(&user.id).await.unwrap();
        assert_eq!(true, delete(&user.id).await.is_err());
    }
}
