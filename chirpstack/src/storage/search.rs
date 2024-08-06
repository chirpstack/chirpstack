use std::collections::HashMap;

use anyhow::{Context, Result};
use diesel_async::RunQueryDsl;
use regex::Regex;
use uuid::Uuid;

use super::{error::Error, fields, get_async_db_conn};
use lrwn::EUI64;

lazy_static! {
    static ref SEARCH_TAG_RE: Regex = Regex::new(r"([^ ]+):([^ ]+)").unwrap();
}

#[derive(QueryableByName, PartialEq, Debug)]
pub struct SearchResult {
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub kind: String,
    #[diesel(sql_type = diesel::sql_types::Float)]
    pub score: f32,
    #[diesel(sql_type = diesel::sql_types::Nullable<fields::sql_types::Uuid>)]
    pub tenant_id: Option<fields::Uuid>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub tenant_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<fields::sql_types::Uuid>)]
    pub application_id: Option<fields::Uuid>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub application_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Binary>)]
    pub device_dev_eui: Option<EUI64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub device_name: Option<String>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Binary>)]
    pub gateway_id: Option<EUI64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::Text>)]
    pub gateway_name: Option<String>,
}

#[cfg(feature = "postgres")]
pub async fn global_search(
    user_id: &Uuid,
    global_admin: bool,
    search: &str,
    limit: usize,
    offset: usize,
) -> Result<Vec<SearchResult>, Error> {
    let (query, tags) = parse_search_query(search);
    let query = format!("%{}%", query);
    let tags = serde_json::to_value(tags).context("To serde_json value")?;

    let res: Vec<SearchResult> = diesel::sql_query(
        r#"
            -- device
            select
                'device' as kind,
                greatest(similarity(d.name, $1), similarity(encode(d.dev_eui, 'hex'), $1), similarity(encode(d.dev_addr, 'hex'), $1)) as score,
                t.id as tenant_id,
                t.name as tenant_name,
                a.id as application_id,
                a.name as application_name,
                d.dev_eui as device_dev_eui,
                d.name as device_name,
                null as gateway_id,
                null as gateway_name
            from device d
            inner join application a
                on a.id = d.application_id
            inner join tenant t
                on t.id = a.tenant_id
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                ($3 = true or u.id = $4)
                    and (d.name ilike $2 or encode(d.dev_eui, 'hex') ilike $2 or encode(d.dev_addr, 'hex') ilike $2 or ($7 != '{}'::jsonb and d.tags @> $7))
            -- gateway
            union
            select
                'gateway' as kind,
                greatest(similarity(g.name, $1), similarity(encode(g.gateway_id, 'hex'), $1)) as score,
                t.id as tenant_id,
                t.name as tenant_name,
                null as application_id,
                null as application_name,
                null as device_dev_eui,
                null as device_name,
                g.gateway_id as gateway_id,
                g.name as gateway_name
            from
                gateway g
            inner join tenant t
                on t.id = g.tenant_id
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                ($3 = true or u.id = $4)
                and (g.name ilike $2 or encode(g.gateway_id, 'hex') ilike $2 or ($7 != '{}'::jsonb and g.tags @> $7))
            -- tenant
            union
            select
                'tenant' as kind,
                similarity(t.name, $1) as score,
                t.id as tenant_id,
                t.name as tenant_name,
                null as application_id,
                null as application_name,
                null as device_dev_eui,
                null as device_name,
                null as gateway_id,
                null as gateway_name
            from
                tenant t
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                ($3 = true or u.id = $4)
                and t.name ilike $2
            -- application
            union
            select
                'application' as kind,
                similarity(a.name, $1) as score,
                t.id as tenant_id,
                t.name as tenant_name,
                a.id as application_id,
                a.name as application_name,
                null as device_dev_eui,
                null as device_name,
                null as gateway_id,
                null as gateway_name
            from
                application a
            inner join tenant t
                on t.id = a.tenant_id
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                ($3 = true or u.id = $4)
                and a.name ilike $2
            order by
                score desc
            limit $5
            offset $6
        "#)
            .bind::<diesel::sql_types::Text, _>(&search)
            .bind::<diesel::sql_types::Text, _>(&query)
            .bind::<diesel::sql_types::Bool, _>(global_admin)
            .bind::<fields::sql_types::Uuid, _>(&fields::Uuid::from(user_id))
            .bind::<diesel::sql_types::BigInt, _>(limit as i64)
            .bind::<diesel::sql_types::BigInt, _>(offset as i64)
            .bind::<fields::sql_types::JsonT, _>(tags)
            .load(&mut get_async_db_conn().await?).await?;

    Ok(res)
}

#[cfg(feature = "sqlite")]
pub async fn global_search(
    user_id: &Uuid,
    global_admin: bool,
    search: &str,
    limit: usize,
    offset: usize,
) -> Result<Vec<SearchResult>, Error> {
    let (query, tags) = parse_search_query(search);
    let query = format!("%{}%", query);
    let tags = serde_json::to_string(&tags).context("To serde_json string")?;

    let res: Vec<SearchResult> = diesel::sql_query(
        r#"
            -- device
            select
                'device' as kind,
                0.0 as score,
                t.id as tenant_id,
                t.name as tenant_name,
                a.id as application_id,
                a.name as application_name,
                d.dev_eui as device_dev_eui,
                d.name as device_name,
                null as gateway_id,
                null as gateway_name
            from device d
            inner join application a
                on a.id = d.application_id
            inner join tenant t
                on t.id = a.tenant_id
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                (?3 = true or u.id = ?4)
                    and (
                        d.name like ?2 or hex(d.dev_eui) like ?2 or hex(d.dev_addr) like ?2
                        or (
                            ?7 != '{}'
                            and 0 = (
                                -- this makes sure tags are present
                                -- by counting number of different top level json values
                                select
                                    count(*)
                                from json_each(?7) search_tag
                                left join json_each(d.tags) item_tag
                                   on search_tag.key = item_tag.key
                                where
                                    -- `is not` is like `!=` but handles null
                                    search_tag.value is not item_tag.value
                            )
                        )
                    )
            -- gateway
            union
            select
                'gateway' as kind,
                0.0 as score,
                t.id as tenant_id,
                t.name as tenant_name,
                null as application_id,
                null as application_name,
                null as device_dev_eui,
                null as device_name,
                g.gateway_id as gateway_id,
                g.name as gateway_name
            from
                gateway g
            inner join tenant t
                on t.id = g.tenant_id
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                (?3 = true or u.id = ?4)
                and (
                    g.name like ?2 or hex(g.gateway_id) like ?2
                    or (
                        ?7 != '{}'
                        and 0 = (
                            -- this makes sure tags are present
                            -- by counting number of different top level json values
                            select
                                count(*)
                            from json_each(?7) search_tag
                            left join json_each(g.tags) item_tag
                               on search_tag.key = item_tag.key
                            where
                                -- `is not` is like `!=` but handles null
                                search_tag.value is not item_tag.value
                        )
                    )
                )
            -- tenant
            union
            select
                'tenant' as kind,
                0.0 as score,
                t.id as tenant_id,
                t.name as tenant_name,
                null as application_id,
                null as application_name,
                null as device_dev_eui,
                null as device_name,
                null as gateway_id,
                null as gateway_name
            from
                tenant t
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                (?3 = true or u.id = ?4)
                and t.name like ?2
            -- application
            union
            select
                'application' as kind,
                0.0 as score,
                t.id as tenant_id,
                t.name as tenant_name,
                a.id as application_id,
                a.name as application_name,
                null as device_dev_eui,
                null as device_name,
                null as gateway_id,
                null as gateway_name
            from
                application a
            inner join tenant t
                on t.id = a.tenant_id
            left join tenant_user tu
                on tu.tenant_id = t.id
            left join "user" u
                on u.id = tu.user_id
            where
                (?3 = true or u.id = ?4)
                and a.name like ?2
            limit ?5
            offset ?6
        "#,
    )
    // first argument is unused but kept to facilitate diffing with postgres query
    .bind::<diesel::sql_types::Text, _>(&search)
    .bind::<diesel::sql_types::Text, _>(&query)
    .bind::<diesel::sql_types::Bool, _>(global_admin)
    .bind::<fields::sql_types::Uuid, _>(&fields::Uuid::from(user_id))
    .bind::<diesel::sql_types::BigInt, _>(limit as i64)
    .bind::<diesel::sql_types::BigInt, _>(offset as i64)
    .bind::<fields::sql_types::JsonT, _>(tags)
    .load(&mut get_async_db_conn().await?)
    .await?;

    Ok(res)
}

fn parse_search_query(q: &str) -> (String, HashMap<String, String>) {
    let mut tags: HashMap<String, String> = HashMap::new();

    for caps in SEARCH_TAG_RE.captures_iter(q) {
        if caps.len() != 3 {
            continue;
        }

        tags.insert(
            caps.get(1).unwrap().as_str().to_string(),
            caps.get(2).unwrap().as_str().to_string(),
        );
    }

    let query = SEARCH_TAG_RE.replace_all(q, "").trim().to_string();
    (query, tags)
}

#[cfg(test)]
pub mod test {
    use std::str::FromStr;

    use super::*;
    use crate::storage::{application, device, device_profile, gateway, tenant, user};
    use crate::test;

    #[test]
    fn test_parse_search_query() {
        struct Test {
            input: String,
            query: String,
            tags: HashMap<String, String>,
        }

        let tests = vec![
            Test {
                input: "foo bar".into(),
                query: "foo bar".into(),
                tags: HashMap::new(),
            },
            Test {
                input: "foo: bar".into(),
                query: "foo: bar".into(),
                tags: HashMap::new(),
            },
            Test {
                input: "foo:bar".into(),
                query: "".into(),
                tags: [("foo".into(), "bar".into())].iter().cloned().collect(),
            },
            Test {
                input: "test foo:bar".into(),
                query: "test".into(),
                tags: [("foo".into(), "bar".into())].iter().cloned().collect(),
            },
            Test {
                input: "test foo:bar alice:bob".into(),
                query: "test".into(),
                tags: [("foo".into(), "bar".into()), ("alice".into(), "bob".into())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];

        for tst in &tests {
            let (query, tags) = parse_search_query(&tst.input);
            assert_eq!(tst.query, query);
            assert_eq!(tst.tags, tags);
        }
    }

    fn build_tags(tags: &[(&str, &str)]) -> fields::KeyValue {
        fields::KeyValue::new(
            tags.iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    }

    #[tokio::test]
    async fn test_global_search() {
        let _guard = test::prepare().await;

        let u = user::create(user::User {
            email: "test@example.com".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        let a = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        let _gw = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_str("0102030405060708").unwrap(),
            name: "test-gateway".into(),
            tenant_id: t.id.clone(),
            tags: build_tags(&[("common_tag", "value"), ("mytag", "gw_value")]),
            ..Default::default()
        })
        .await
        .unwrap();

        let _d = device::create(device::Device {
            dev_eui: EUI64::from_str("0203040506070809").unwrap(),
            name: "test-device".into(),
            application_id: a.id.clone(),
            device_profile_id: dp.id.clone(),
            tags: build_tags(&[("common_tag", "value"), ("mytag", "dev_value")]),
            ..Default::default()
        })
        .await
        .unwrap();

        let _d = device::create(device::Device {
            dev_eui: EUI64::from_str("030405060708090A").unwrap(),
            name: "sensor".into(),
            application_id: a.id.clone(),
            device_profile_id: dp.id.clone(),
            tags: build_tags(&[("a", "1"), ("b", "2")]),
            ..Default::default()
        })
        .await
        .unwrap();

        // If user is not a global admin, this does not return any results.
        let queries: Vec<String> = vec![
            "test".into(),
            "ten".into(),
            "app".into(),
            "010203".into(),
            "020304".into(),
            "device".into(),
            "other mytag:gw_value".into(),
            "other mytag:dev_value".into(),
            "other common_tag:value".into(),
        ];
        for q in &queries {
            println!("{q}");
            let res = global_search(&u.id, false, q, 10, 0).await.unwrap();
            assert_eq!(0, res.len());
        }

        // If user is a global admin, this returns results.
        let queries: HashMap<String, usize> = [
            ("test".into(), 4),
            ("ten".into(), 1),
            ("app".into(), 1),
            ("010203".into(), 1),
            ("020304".into(), 2),
            ("device".into(), 1),
            ("dev".into(), 1),
            ("gatew".into(), 1),
            ("other mytag:gw_value".into(), 1),
            ("other mytag:dev_value".into(), 1),
            ("other common_tag:value".into(), 2),
            ("other a:1 b:2".into(), 1),
        ]
        .iter()
        .cloned()
        .collect();
        for (k, v) in &queries {
            let res = global_search(&u.id, true, k, 10, 0).await.unwrap();
            println!("{res:#?}");
            assert_eq!(*v, res.len(), "query: {}", k);
        }

        // User is tenant-user, this returns results.
        tenant::add_user(tenant::TenantUser {
            tenant_id: t.id.clone(),
            user_id: u.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        let queries: HashMap<String, usize> = [
            ("test".into(), 4),
            ("ten".into(), 1),
            ("app".into(), 1),
            ("010203".into(), 1),
            ("020304".into(), 2),
            ("device".into(), 1),
            ("dev".into(), 1),
            ("gatew".into(), 1),
            ("other mytag:gw_value".into(), 1),
            ("other mytag:dev_value".into(), 1),
            ("other common_tag:value".into(), 2),
            ("other a:1 b:2".into(), 1),
        ]
        .iter()
        .cloned()
        .collect();
        for (k, v) in &queries {
            let res = global_search(&u.id, false, k, 10, 0).await.unwrap();
            assert_eq!(*v, res.len(), "query: {}", k);
        }
    }
}
