use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::backend::Backend;
use diesel::dsl;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::{Jsonb, Text};
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use tokio::task;
use tracing::info;
use uuid::Uuid;

use super::error::Error;
use super::get_db_conn;
use super::schema::{application, application_integration};

#[derive(Clone, Queryable, Insertable, PartialEq, Eq, Debug)]
#[diesel(table_name = application)]
pub struct Application {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub mqtt_tls_cert: Option<Vec<u8>>,
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
            id: Uuid::new_v4(),
            tenant_id: Uuid::nil(),
            created_at: now,
            updated_at: now,
            name: "".into(),
            description: "".into(),
            mqtt_tls_cert: None,
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
    pub id: Uuid,
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
    LoraCloud,
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
            "LoraCloud" => IntegrationKind::LoraCloud,
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
        let string = String::from_sql(value)?;
        Ok(IntegrationKind::from_str(&string)?)
    }
}

impl serialize::ToSql<Text, Pg> for IntegrationKind
where
    str: serialize::ToSql<Text, Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        <str as serialize::ToSql<Text, Pg>>::to_sql(&self.to_string(), &mut out.reborrow())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Jsonb)]
pub enum IntegrationConfiguration {
    None,
    Http(HttpConfiguration),
    InfluxDb(InfluxDbConfiguration),
    ThingsBoard(ThingsBoardConfiguration),
    MyDevices(MyDevicesConfiguration),
    LoraCloud(LoraCloudConfiguration),
    GcpPubSub(GcpPubSubConfiguration),
    AwsSns(AwsSnsConfiguration),
    AzureServiceBus(AzureServiceBusConfiguration),
    PilotThings(PilotThingsConfiguration),
    Ifttt(IftttConfiguration),
}

impl deserialize::FromSql<Jsonb, Pg> for IntegrationConfiguration {
    fn from_sql(value: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as deserialize::FromSql<Jsonb, Pg>>::from_sql(value)?;
        Ok(serde_json::from_value(value)?)
    }
}

impl serialize::ToSql<Jsonb, Pg> for IntegrationConfiguration {
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, Pg>) -> serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as serialize::ToSql<Jsonb, Pg>>::to_sql(&value, &mut out.reborrow())
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
pub struct LoraCloudConfiguration {
    pub modem_geolocation_services: LoraCloudModemGeolocationServices,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LoraCloudModemGeolocationServices {
    pub token: String,
    pub modem_enabled: bool,
    pub modem_port: u32,
    pub gnss_port: u32,
    pub forward_f_ports: Vec<u32>,
    pub gnss_use_rx_time: bool,
    pub gnss_use_gateway_location: bool,
    pub parse_tlv: bool,
    pub geolocation_buffer_ttl: u32,
    pub geolocation_min_buffer_size: u32,
    pub geolocation_tdoa: bool,
    pub geolocation_rssi: bool,
    pub geolocation_gnss: bool,
    pub geolocation_gnss_payload_field: String,
    pub geolocation_gnss_use_rx_time: bool,
    pub geolocation_wifi: bool,
    pub geolocation_wifi_payload_field: String,
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
    pub application_id: Uuid,
    pub kind: IntegrationKind,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub configuration: IntegrationConfiguration,
}

impl Default for Integration {
    fn default() -> Self {
        let now = Utc::now();

        Integration {
            application_id: Uuid::nil(),
            kind: IntegrationKind::Http,
            created_at: now,
            updated_at: now,
            configuration: IntegrationConfiguration::None,
        }
    }
}

pub async fn create(a: Application) -> Result<Application, Error> {
    a.validate()?;
    task::spawn_blocking({
        move || -> Result<Application, Error> {
            let mut c = get_db_conn()?;
            let a: Application = diesel::insert_into(application::table)
                .values(&a)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, a.id.to_string()))?;

            info!(id = %a.id, "Application created");

            Ok(a)
        }
    })
    .await?
}

pub async fn get(id: &Uuid) -> Result<Application, Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<Application, Error> {
            let mut c = get_db_conn()?;
            let a = application::dsl::application
                .find(&id)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;
            Ok(a)
        }
    })
    .await?
}

pub async fn update(a: Application) -> Result<Application, Error> {
    a.validate()?;
    task::spawn_blocking({
        move || -> Result<Application, Error> {
            let mut c = get_db_conn()?;
            let a: Application = diesel::update(application::dsl::application.find(&a.id))
                .set((
                    application::updated_at.eq(Utc::now()),
                    application::name.eq(&a.name),
                    application::description.eq(&a.description),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, a.id.to_string()))?;

            info!(
                application_id = %a.id,
                "Application updated"
            );

            Ok(a)
        }
    })
    .await?
}

pub async fn update_mqtt_cls_cert(id: &Uuid, cert: &[u8]) -> Result<Application, Error> {
    let app = task::spawn_blocking({
        let id = *id;
        let cert = cert.to_vec();
        move || -> Result<Application, Error> {
            let mut c = get_db_conn()?;
            let app: Application = diesel::update(application::dsl::application.find(&id))
                .set(application::mqtt_tls_cert.eq(cert))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;
            Ok(app)
        }
    })
    .await??;

    info!(
        application_id = %id,
        "Application MQTT certificate updated"
    );

    Ok(app)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(application::dsl::application.find(&id)).execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(id.to_string()));
            }

            info!(
                application_id = %id,
                "Application deleted"
            );

            Ok(())
        }
    })
    .await?
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = application::dsl::application
                .select(dsl::count_star())
                .into_boxed();

            if let Some(tenant_id) = &filters.tenant_id {
                q = q.filter(application::dsl::tenant_id.eq(tenant_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(application::dsl::name.ilike(format!("%{}%", search)));
            }

            Ok(q.first(&mut c)?)
        }
    })
    .await?
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
) -> Result<Vec<ApplicationListItem>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<ApplicationListItem>, Error> {
            let mut c = get_db_conn()?;
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
                q = q.filter(application::dsl::tenant_id.eq(tenant_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(application::dsl::name.ilike(format!("%{}%", search)));
            }

            let items = q
                .order_by(application::dsl::name)
                .limit(limit)
                .offset(offset)
                .load(&mut c)?;
            Ok(items)
        }
    })
    .await?
}

pub async fn create_integration(i: Integration) -> Result<Integration, Error> {
    task::spawn_blocking({
        move || -> Result<Integration, Error> {
            let mut c = get_db_conn()?;
            let i: Integration = diesel::insert_into(application_integration::table)
                .values(&i)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, i.kind.to_string()))?;

            info!(application_id = %i.application_id, kind = %i.kind, "Integration created");
            Ok(i)
        }
    })
    .await?
}

pub async fn get_integration(
    application_id: &Uuid,
    kind: IntegrationKind,
) -> Result<Integration, Error> {
    task::spawn_blocking({
        let application_id = *application_id;
        move || -> Result<Integration, Error> {
            let mut c = get_db_conn()?;
            let mut i: Integration = application_integration::dsl::application_integration
                .filter(
                    application_integration::dsl::application_id
                        .eq(application_id)
                        .and(application_integration::dsl::kind.eq(kind)),
                )
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;

            // For backwards compatibiliy
            if let IntegrationConfiguration::LoraCloud(conf) = &mut i.configuration {
                if conf.modem_geolocation_services.forward_f_ports.is_empty() {
                    conf.modem_geolocation_services.forward_f_ports = vec![
                        conf.modem_geolocation_services.modem_port,
                        conf.modem_geolocation_services.gnss_port,
                        197,
                        192,
                    ];
                }
            }

            Ok(i)
        }
    })
    .await?
}

pub async fn update_integration(i: Integration) -> Result<Integration, Error> {
    task::spawn_blocking({
        move || -> Result<Integration, Error> {
            let mut c = get_db_conn()?;
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
            .get_result(&mut c)
            .map_err(|e| Error::from_diesel(e, i.application_id.to_string()))?;

            info!(application_id = %i.application_id, kind = %i.kind, "Integration updated");

            Ok(i)
        }
    })
    .await?
}

pub async fn delete_integration(application_id: &Uuid, kind: IntegrationKind) -> Result<(), Error> {
    task::spawn_blocking({
        let application_id = *application_id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(
                application_integration::dsl::application_integration.filter(
                    application_integration::dsl::application_id
                        .eq(&application_id)
                        .and(application_integration::dsl::kind.eq(&kind)),
                ),
            )
            .execute(&mut c)?;

            if ra == 0 {
                return Err(Error::NotFound(application_id.to_string()));
            }

            info!(application_id = %application_id, kind = %kind, "Integration deleted");
            Ok(())
        }
    })
    .await?
}

pub async fn get_integrations_for_application(
    application_id: &Uuid,
) -> Result<Vec<Integration>, Error> {
    task::spawn_blocking({
        let application_id = *application_id;
        move || -> Result<Vec<Integration>, Error> {
            let mut c = get_db_conn()?;
            let items: Vec<Integration> = application_integration::dsl::application_integration
                .filter(application_integration::dsl::application_id.eq(&application_id))
                .order_by(application_integration::dsl::kind)
                .load(&mut c)?;
            Ok(items)
        }
    })
    .await?
}

pub async fn get_measurement_keys(application_id: &Uuid) -> Result<Vec<String>, Error> {
    #[derive(QueryableByName)]
    struct Measurement {
        #[diesel(sql_type = diesel::sql_types::Text)]
        pub key: String,
    }

    task::spawn_blocking({
        let application_id = *application_id;
        move || -> Result<Vec<String>, Error> {
            let mut c = get_db_conn()?;
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
            .bind::<diesel::sql_types::Uuid, _>(application_id)
            .load(&mut c)
            .map_err(|e| Error::from_diesel(e, application_id.to_string()))?;
            Ok(keys.iter().map(|k| k.key.clone()).collect())
        }
    })
    .await?
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
            Some(v) => v,
            None => {
                let t = storage::tenant::test::create_tenant().await;
                t.id
            }
        };

        let a = Application {
            tenant_id: tenant_id,
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
                    tenant_id: Some(app.tenant_id),
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
        assert_eq!(true, delete(&app.id).await.is_err());
    }
}
