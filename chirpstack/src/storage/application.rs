use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use chrono::{DateTime, Utc};
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
use diesel::{backend::Backend, deserialize, dsl, prelude::*, serialize, sql_types::Text};
#[cfg(feature = "postgres")]
use diesel::{pg::Pg, sql_types::Jsonb};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use super::error::Error;
use super::schema::{application, application_integration, device, device_profile};
use super::{fields, get_async_db_conn};

#[derive(Clone, Queryable, Insertable, PartialEq, Eq, Debug)]
#[diesel(table_name = application)]
pub struct Application {
    pub id: fields::Uuid,
    pub tenant_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub mqtt_tls_cert: Option<Vec<u8>>,
    pub tags: fields::KeyValue,
}

impl Application {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }
        Ok(())
    }
}

impl Default for Application {
    fn default() -> Self {
        let now = Utc::now();

        Application {
            id: Uuid::new_v4().into(),
            tenant_id: Uuid::nil().into(),
            created_at: now,
            updated_at: now,
            name: "".into(),
            description: "".into(),
            mqtt_tls_cert: None,
            tags: fields::KeyValue::new(HashMap::new()),
        }
    }
}

#[derive(Default, Clone)]
pub struct Filters {
    pub tenant_id: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct ApplicationListItem {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum IntegrationKind {
    Http,
    InfluxDb,
    ThingsBoard,
    MyDevices,
    GcpPubSub,
    AwsSns,
    AzureServiceBus,
    PilotThings,
    Ifttt,
}

impl fmt::Display for IntegrationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for IntegrationKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "Http" => IntegrationKind::Http,
            "InfluxDb" => IntegrationKind::InfluxDb,
            "ThingsBoard" => IntegrationKind::ThingsBoard,
            "MyDevices" => IntegrationKind::MyDevices,
            "GcpPubSub" => IntegrationKind::GcpPubSub,
            "AwsSns" => IntegrationKind::AwsSns,
            "AzureServiceBus" => IntegrationKind::AzureServiceBus,
            "PilotThings" => IntegrationKind::PilotThings,
            "Ifttt" => IntegrationKind::Ifttt,
            _ => {
                return Err(anyhow!("Unexpected IntegrationKind: {}", s));
            }
        })
    }
}

impl<DB> deserialize::FromSql<Text, DB> for IntegrationKind
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = <*const str>::from_sql(value)?;
        Ok(Self::from_str(unsafe { &*string })?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Text, Pg> for IntegrationKind
where
    str: serialize::ToSql<Text, Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        <str as serialize::ToSql<Text, Pg>>::to_sql(&self.to_string(), &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for IntegrationKind {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.to_string());
        Ok(serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = fields::sql_types::JsonT)]
pub enum IntegrationConfiguration {
    None,
    Http(HttpConfiguration),
    InfluxDb(InfluxDbConfiguration),
    ThingsBoard(ThingsBoardConfiguration),
    MyDevices(MyDevicesConfiguration),
    GcpPubSub(GcpPubSubConfiguration),
    AwsSns(AwsSnsConfiguration),
    AzureServiceBus(AzureServiceBusConfiguration),
    PilotThings(PilotThingsConfiguration),
    Ifttt(IftttConfiguration),
}

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Jsonb, Pg> for IntegrationConfiguration {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Jsonb, Pg> for IntegrationConfiguration {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for IntegrationConfiguration {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let s =
            <*const str as deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(value)?;
        Ok(serde_json::from_str(unsafe { &*s })?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for IntegrationConfiguration {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Sqlite>) -> serialize::Result {
        out.set_value(serde_json::to_string(self)?);
        Ok(serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpConfiguration {
    pub headers: HashMap<String, String>,
    pub json: bool,
    pub event_endpoint_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfluxDbConfiguration {
    pub endpoint: String,
    pub db: String,
    pub username: String,
    pub password: String,
    pub retention_policy_name: String,
    pub precision: i32,
    pub version: i32,
    pub token: String,
    pub organization: String,
    pub bucket: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThingsBoardConfiguration {
    pub server: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MyDevicesConfiguration {
    pub endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GcpPubSubConfiguration {
    pub encoding: i32,
    pub credentials_file: String,
    pub project_id: String,
    pub topic_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AwsSnsConfiguration {
    pub encoding: i32,
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub topic_arn: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AzureServiceBusConfiguration {
    pub encoding: i32,
    pub connection_string: String,
    pub publish_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PilotThingsConfiguration {
    pub server: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct IftttConfiguration {
    pub key: String,
    pub uplink_values: [String; 2], // The first value is reserved for the DevEUI
    pub arbitrary_json: bool,
    pub event_prefix: String,
}

#[derive(Clone, Queryable, Insertable, PartialEq, Eq, Debug)]
#[diesel(table_name = application_integration)]
pub struct Integration {
    pub application_id: fields::Uuid,
    pub kind: IntegrationKind,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub configuration: IntegrationConfiguration,
}

impl Default for Integration {
    fn default() -> Self {
        let now = Utc::now();

        Integration {
            application_id: Uuid::nil().into(),
            kind: IntegrationKind::Http,
            created_at: now,
            updated_at: now,
            configuration: IntegrationConfiguration::None,
        }
    }
}

pub async fn create(a: Application) -> Result<Application, Error> {
    a.validate()?;

    let a: Application = diesel::insert_into(application::table)
        .values(&a)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, a.id.to_string()))?;

    info!(id = %a.id, "Application created");

    Ok(a)
}

pub async fn get(id: &Uuid) -> Result<Application, Error> {
    let a = application::dsl::application
        .find(fields::Uuid::from(id))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, id.to_string()))?;
    Ok(a)
}

pub async fn update(a: Application) -> Result<Application, Error> {
    a.validate()?;

    let a: Application = diesel::update(application::dsl::application.find(&a.id))
        .set((
            application::updated_at.eq(Utc::now()),
            application::name.eq(&a.name),
            application::description.eq(&a.description),
            application::tags.eq(&a.tags),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, a.id.to_string()))?;

    info!(
        application_id = %a.id,
        "Application updated"
    );

    Ok(a)
}

pub async fn update_mqtt_cls_cert(id: &Uuid, cert: &[u8]) -> Result<Application, Error> {
    let app: Application =
        diesel::update(application::dsl::application.find(fields::Uuid::from(id)))
            .set(application::mqtt_tls_cert.eq(cert))
            .get_result(&mut get_async_db_conn().await?)
            .await
            .map_err(|e| Error::from_diesel(e, id.to_string()))?;

    info!(
        application_id = %id,
        "Application MQTT certificate updated"
    );

    Ok(app)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    let ra = diesel::delete(application::dsl::application.find(fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await?;
    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }

    info!(
        application_id = %id,
        "Application deleted"
    );

    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    let mut q = application::dsl::application
        .select(dsl::count_star())
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(application::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(application::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(application::dsl::name.like(format!("%{}%", search)));
        }
    }

    Ok(q.first(&mut get_async_db_conn().await?).await?)
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
) -> Result<Vec<ApplicationListItem>, Error> {
    let mut q = application::dsl::application
        .select((
            application::id,
            application::created_at,
            application::updated_at,
            application::name,
            application::description,
        ))
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(application::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(application::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(application::dsl::name.like(format!("%{}%", search)));
        }
    }

    let items = q
        .order_by(application::dsl::name)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await?;
    Ok(items)
}

pub async fn create_integration(i: Integration) -> Result<Integration, Error> {
    let i: Integration = diesel::insert_into(application_integration::table)
        .values(&i)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, i.kind.to_string()))?;

    info!(application_id = %i.application_id, kind = %i.kind, "Integration created");
    Ok(i)
}

pub async fn get_integration(
    application_id: &Uuid,
    kind: IntegrationKind,
) -> Result<Integration, Error> {
    let i: Integration = application_integration::dsl::application_integration
        .filter(
            application_integration::dsl::application_id
                .eq(fields::Uuid::from(application_id))
                .and(application_integration::dsl::kind.eq(kind)),
        )
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;

    Ok(i)
}

pub async fn update_integration(i: Integration) -> Result<Integration, Error> {
    let i: Integration = diesel::update(
        application_integration::dsl::application_integration.filter(
            application_integration::dsl::application_id
                .eq(&i.application_id)
                .and(application_integration::dsl::kind.eq(&i.kind)),
        ),
    )
    .set((
        application_integration::updated_at.eq(Utc::now()),
        application_integration::configuration.eq(&i.configuration),
    ))
    .get_result(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, i.application_id.to_string()))?;

    info!(application_id = %i.application_id, kind = %i.kind, "Integration updated");

    Ok(i)
}

pub async fn delete_integration(application_id: &Uuid, kind: IntegrationKind) -> Result<(), Error> {
    let ra = diesel::delete(
        application_integration::dsl::application_integration.filter(
            application_integration::dsl::application_id
                .eq(fields::Uuid::from(application_id))
                .and(application_integration::dsl::kind.eq(&kind)),
        ),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;

    if ra == 0 {
        return Err(Error::NotFound(application_id.to_string()));
    }

    info!(application_id = %application_id, kind = %kind, "Integration deleted");
    Ok(())
}

pub async fn get_integrations_for_application(
    application_id: &Uuid,
) -> Result<Vec<Integration>, Error> {
    let items: Vec<Integration> = application_integration::dsl::application_integration
        .filter(application_integration::dsl::application_id.eq(fields::Uuid::from(application_id)))
        .order_by(application_integration::dsl::kind)
        .load(&mut get_async_db_conn().await?)
        .await?;
    Ok(items)
}

#[derive(QueryableByName)]
struct Measurement {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub key: String,
}

#[cfg(feature = "postgres")]
pub async fn get_measurement_keys(application_id: &Uuid) -> Result<Vec<String>, Error> {
    let keys: Vec<Measurement> = diesel::sql_query(
        r#"
                select
                    distinct jsonb_object_keys(dp.measurements) as key
                from
                    device_profile dp
                inner join device d
                    on d.device_profile_id = dp.id
                where
                    d.application_id = $1
                order by
                    key
                "#,
    )
    .bind::<fields::sql_types::Uuid, _>(fields::Uuid::from(application_id))
    .load(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;
    Ok(keys.iter().map(|k| k.key.clone()).collect())
}

#[cfg(feature = "sqlite")]
pub async fn get_measurement_keys(application_id: &Uuid) -> Result<Vec<String>, Error> {
    let keys: Vec<Measurement> = diesel::sql_query(
        r#"
                    select distinct json_each.key as key
                    from device_profile dp, json_each(dp.measurements)
                    inner join device d
                        on d.device_profile_id = dp.id
                    where
                        d.application_id = ?
                    order by
                        key
                    "#,
    )
    .bind::<fields::sql_types::Uuid, _>(fields::Uuid::from(application_id))
    .load(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;
    Ok(keys.iter().map(|k| k.key.clone()).collect())
}

pub async fn get_device_profiles(
    application_id: Uuid,
) -> Result<Vec<(fields::Uuid, String)>, Error> {
    let result: Vec<(fields::Uuid, String)> = device_profile::dsl::device_profile
        .select((device_profile::dsl::id, device_profile::dsl::name))
        .distinct()
        .inner_join(device::table.on(device::dsl::device_profile_id.eq(device_profile::dsl::id)))
        .filter(device::dsl::application_id.eq(fields::Uuid::from(application_id)))
        .order_by(device_profile::dsl::name)
        .load(&mut get_async_db_conn().await?)
        .await?;

    Ok(result)
}

#[derive(QueryableByName)]
struct DeviceTags {
    #[diesel(sql_type = diesel::sql_types::Text)]
    key: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    value: String,
}

#[cfg(feature = "postgres")]
pub async fn get_device_tags(application_id: Uuid) -> Result<BTreeMap<String, Vec<String>>, Error> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let items: Vec<DeviceTags> = diesel::sql_query(
        r#"
            select
                distinct
                    t.key,
                    t.value
                from device d
                join lateral jsonb_each_text(d.tags) t
                    on true
                where
                    d.application_id = $1
                order by
                    t.key,
                    t.value
        "#,
    )
    .bind::<fields::sql_types::Uuid, _>(fields::Uuid::from(application_id))
    .load(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;

    for item in &items {
        let entry = out.entry(item.key.clone()).or_default();
        entry.push(item.value.clone());
    }

    Ok(out)
}

#[cfg(feature = "sqlite")]
pub async fn get_device_tags(application_id: Uuid) -> Result<BTreeMap<String, Vec<String>>, Error> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let items: Vec<DeviceTags> = diesel::sql_query(
        r#"
            select
                distinct
                    t.key,
                    t.value
                from
                    device d,
                    json_each(d.tags) as t
                where
                    d.application_id = ?1
                order by
                    t.key,
                    t.value
        "#,
    )
    .bind::<fields::sql_types::Uuid, _>(fields::Uuid::from(application_id))
    .load(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;

    for item in &items {
        let entry = out.entry(item.key.clone()).or_default();
        entry.push(item.value.clone());
    }

    Ok(out)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage;
    use crate::test;

    struct FilterTest<'a> {
        filters: Filters,
        apps: Vec<&'a Application>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    pub async fn create_application(tenant_id: Option<Uuid>) -> Application {
        let tenant_id = match tenant_id {
            Some(v) => v.into(),
            None => {
                let t = storage::tenant::test::create_tenant().await;
                t.id
            }
        };

        let a = Application {
            tenant_id,
            name: "test application".into(),
            description: "test application description".into(),
            ..Default::default()
        };
        create(a).await.unwrap()
    }

    #[tokio::test]
    async fn test_application() {
        let _guard = test::prepare().await;
        let mut app = create_application(None).await;

        // get
        let app_get = get(&app.id).await.unwrap();
        assert_eq!(app, app_get);

        // update
        app.name = "update application".into();
        app = update(app).await.unwrap();
        let app_get = get(&app.id).await.unwrap();
        assert_eq!(app, app_get);

        // get count and list
        let tests = vec![
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: None,
                },
                apps: vec![&app],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: Some("aap".into()),
                },
                apps: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: Some("app".into()),
                },
                apps: vec![&app],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: Some("app".into()),
                },
                apps: vec![],
                count: 1,
                limit: 0,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(app.tenant_id.into()),
                    search: None,
                },
                apps: vec![&app],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(Uuid::new_v4()),
                    search: None,
                },
                apps: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
        ];

        for tst in tests {
            let count = get_count(&tst.filters).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(tst.limit, tst.offset, &tst.filters).await.unwrap();
            assert_eq!(
                tst.apps
                    .iter()
                    .map(|app| { app.id.to_string() })
                    .collect::<String>(),
                items
                    .iter()
                    .map(|app| { app.id.to_string() })
                    .collect::<String>()
            );
        }

        // delete
        delete(&app.id).await.unwrap();
        assert!(delete(&app.id).await.is_err());
    }
}
