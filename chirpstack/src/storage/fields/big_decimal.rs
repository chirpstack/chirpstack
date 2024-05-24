use diesel::{
    backend::Backend,
    {deserialize, serialize},
};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, sql_types::Numeric};
#[cfg(feature = "sqlite")]
use diesel::{sql_types::Double, sqlite::Sqlite};

#[derive(Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
// Sqlite has no native arbitrary precision type so use double for now
// TODO: Maybe use string representation instead?
#[cfg_attr(feature="postgres", diesel(sql_type = Numeric))]
#[cfg_attr(feature="sqlite", diesel(sql_type = Double))]
pub struct BigDecimal(bigdecimal::BigDecimal);

impl std::convert::AsRef<bigdecimal::BigDecimal> for BigDecimal {
    fn as_ref(&self) -> &bigdecimal::BigDecimal {
        &self.0
    }
}

impl std::convert::From<bigdecimal::BigDecimal> for BigDecimal {
    fn from(value: bigdecimal::BigDecimal) -> Self {
        Self(value)
    }
}

impl std::convert::TryFrom<f32> for BigDecimal {
    type Error = <bigdecimal::BigDecimal as TryFrom<f32>>::Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        bigdecimal::BigDecimal::try_from(value).map(|bd| bd.into())
    }
}

impl std::ops::Deref for BigDecimal {
    type Target = bigdecimal::BigDecimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BigDecimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Numeric, Pg> for BigDecimal {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let u = <bigdecimal::BigDecimal>::from_sql(value)?;
        Ok(BigDecimal(u))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Numeric, Pg> for BigDecimal {
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <bigdecimal::BigDecimal as serialize::ToSql<Numeric, Pg>>::to_sql(
            &self.0,
            &mut out.reborrow(),
        )
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Double, Sqlite> for BigDecimal
where
    f64: deserialize::FromSql<Double, Sqlite>,
{
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        use bigdecimal::FromPrimitive;
        let bd_val =
            <f64 as deserialize::FromSql<diesel::sql_types::Double, Sqlite>>::from_sql(value)?;
        let bd = bigdecimal::BigDecimal::from_f64(bd_val)
            .ok_or_else(|| format!("Unrepresentable BigDecimal from f64 value"))?;
        Ok(BigDecimal(bd))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Double, Sqlite> for BigDecimal {
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        use bigdecimal::ToPrimitive;
        let value = self
            .0
            .to_f64()
            .ok_or_else(|| format!("Unrepresentable f64 value as BigDecimal"))?;
        out.set_value(value);
        Ok(serialize::IsNull::No)
    }
}
