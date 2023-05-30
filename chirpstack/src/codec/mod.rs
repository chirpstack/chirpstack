use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use diesel::backend::Backend;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

mod cayenne_lpp;
pub mod convert;
mod js;

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum Codec {
    NONE,
    CAYENNE_LPP,
    JS,
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Codec
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(value)?;
        Ok(Codec::from_str(&string)?)
    }
}

impl serialize::ToSql<Text, Pg> for Codec
where
    str: serialize::ToSql<Text, Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        <str as serialize::ToSql<Text, Pg>>::to_sql(&self.to_string(), &mut out.reborrow())
    }
}

impl FromStr for Codec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "" | "NONE" => Codec::NONE,
            "CAYENNE_LPP" => Codec::CAYENNE_LPP,
            "JS" => Codec::JS,
            _ => {
                return Err(anyhow!("Unexpected codec: {}", s));
            }
        })
    }
}

pub async fn binary_to_struct(
    codec: Codec,
    recv_time: DateTime<Utc>,
    f_port: u8,
    variables: &HashMap<String, String>,
    decoder_config: &str,
    b: &[u8],
) -> Result<Option<pbjson_types::Struct>> {
    Ok(match codec {
        Codec::NONE => None,
        Codec::CAYENNE_LPP => Some(cayenne_lpp::decode(b).context("CayenneLpp decode")?),
        Codec::JS => Some(js::decode(recv_time, f_port, variables, decoder_config, b).await?),
    })
}

pub async fn struct_to_binary(
    codec: Codec,
    f_port: u8,
    variables: &HashMap<String, String>,
    encoder_config: &str,
    obj: &prost_types::Struct,
) -> Result<Vec<u8>> {
    Ok(match codec {
        Codec::NONE => Vec::new(),
        Codec::CAYENNE_LPP => cayenne_lpp::encode(obj).context("CayenneLpp encode")?,
        Codec::JS => js::encode(f_port, variables, encoder_config, obj).await?,
    })
}

pub fn get_measurements(s: &pbjson_types::Struct) -> HashMap<String, pbjson_types::value::Kind> {
    let mut out: HashMap<String, pbjson_types::value::Kind> = HashMap::new();

    for (k, v) in &s.fields {
        out.extend(_get_measurements(k, v));
    }

    out
}

fn _get_measurements(
    prefix: &str,
    v: &pbjson_types::Value,
) -> HashMap<String, pbjson_types::value::Kind> {
    let mut out: HashMap<String, pbjson_types::value::Kind> = HashMap::new();

    match &v.kind {
        None => {}
        Some(v) => match v {
            pbjson_types::value::Kind::NullValue(_) => {}
            pbjson_types::value::Kind::NumberValue(_)
            | pbjson_types::value::Kind::StringValue(_)
            | pbjson_types::value::Kind::BoolValue(_) => {
                out.insert(prefix.to_string(), v.clone());
            }
            pbjson_types::value::Kind::StructValue(v) => {
                for (k, v) in &v.fields {
                    out.extend(_get_measurements(&format!("{}_{}", prefix, k), v));
                }
            }
            pbjson_types::value::Kind::ListValue(v) => {
                for (i, v) in v.values.iter().enumerate() {
                    out.extend(_get_measurements(&format!("{}_{}", prefix, i), v));
                }
            }
        },
    }

    out
}
