use std::collections::HashSet;
use std::ops::Add;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AuthClaim {
    pub aud: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub exp: Option<usize>,
    pub iss: String,
    pub sub: String,
    pub typ: String,
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

impl AuthClaim {
    pub fn new_for_user(id: &Uuid) -> Self {
        let nbf: DateTime<Utc> = Utc::now();
        let exp = nbf.add(Duration::days(1));

        AuthClaim {
            aud: "chirpstack".to_string(),
            exp: Some(exp.timestamp() as usize),
            iss: "chirpstack".to_string(),
            sub: id.to_string(),
            typ: "user".to_string(),
        }
    }

    pub fn new_for_api_key(id: &Uuid) -> Self {
        AuthClaim {
            aud: "chirpstack".to_string(),
            iss: "chirpstack".to_string(),
            sub: id.to_string(),
            typ: "key".to_string(),
            exp: None,
        }
    }

    pub fn encode(&self, secret: &[u8]) -> Result<String> {
        Ok(encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret),
        )?)
    }

    pub fn decode(token: &str, secret: &[u8]) -> Result<Self> {
        let mut val = Validation::new(Algorithm::HS256);
        val.required_spec_claims = HashSet::new(); // make the 'exp' optional

        let claim = decode::<AuthClaim>(token, &DecodingKey::from_secret(secret), &val)?;
        Ok(claim.claims)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_for_user() {
        let secrect = &"verysecret";
        let other_secret = &"notsosecret";
        let user_id = Uuid::new_v4();
        let key_id = Uuid::new_v4();

        let nbf: DateTime<Utc> = Utc::now();
        let exp = nbf.add(-Duration::days(1));

        let claim = AuthClaim::new_for_api_key(&key_id);
        assert_eq!("key", claim.typ);
        assert_eq!(key_id.to_string(), claim.sub);

        let token = claim.encode(secrect.as_ref()).unwrap();
        let decoded = AuthClaim::decode(&token, secrect.as_ref()).unwrap();
        assert_eq!(claim, decoded);

        // user token
        let mut claim = AuthClaim::new_for_user(&user_id);
        assert_eq!("user", claim.typ);
        assert_eq!(user_id.to_string(), claim.sub);

        let token = claim.encode(secrect.as_ref()).unwrap();
        let decoded = AuthClaim::decode(&token, secrect.as_ref()).unwrap();
        assert_eq!(claim, decoded);

        // different key
        assert_eq!(
            true,
            AuthClaim::decode(&token, other_secret.as_ref()).is_err()
        );

        // expired
        claim.exp = Some(exp.timestamp() as usize);
        let token = claim.encode(secrect.as_ref()).unwrap();
        assert_eq!(true, AuthClaim::decode(&token, secrect.as_ref()).is_err());
    }
}
