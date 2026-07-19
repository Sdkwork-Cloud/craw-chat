//! Isolated PostgreSQL lifecycle coverage for the active IM Agents contract.

use std::time::{SystemTime, UNIX_EPOCH};

use postgres::{Client, NoTls};
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};

struct IsolatedSchema {
    admin_url: String,
    schema: String,
}

impl Drop for IsolatedSchema {
    fn drop(&mut self) {
        let admin_url = self.admin_url.clone();
        let schema = self.schema.clone();
        let _ = std::thread::spawn(move || {
            let Ok(mut client) = Client::connect(&admin_url, NoTls) else {
                return;
            };
            let _ = client.batch_execute(&format!("DROP SCHEMA IF EXISTS {schema} CASCADE"));
        })
        .join();
    }
}

fn isolated_url(base_url: &str, schema: &str) -> String {
    let separator = if base_url.contains('?') { '&' } else { '?' };
    format!("{base_url}{separator}options=-c%20search_path%3D{schema}")
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live PostgreSQL via SDKWORK_IM_DATABASE_URL with schema create/drop permission"]
async fn agents_contract_bootstraps_through_0006_in_an_isolated_schema() {
    let base_url = std::env::var("SDKWORK_IM_DATABASE_URL")
        .expect("SDKWORK_IM_DATABASE_URL must be set for live migration test");
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after epoch")
        .as_millis();
    let schema = format!("im_agents_live_{suffix}");
    let admin_url = base_url.clone();
    let schema_to_create = schema.clone();
    tokio::task::spawn_blocking(move || {
        let mut admin = Client::connect(&admin_url, NoTls).expect("postgres admin should connect");
        admin
            .batch_execute(&format!("CREATE SCHEMA {schema_to_create}"))
            .expect("isolated schema should be created");
    })
    .await
    .expect("schema creation task should complete");
    let _schema_guard = IsolatedSchema {
        admin_url: base_url.clone(),
        schema: schema.clone(),
    };

    let database_url = isolated_url(&base_url, &schema);
    let pool = sdkwork_database_sqlx::create_pool_from_config(DatabaseConfig {
        engine: DatabaseEngine::Postgres,
        url: database_url.clone(),
        max_connections: 4,
        min_connections: 1,
        ..DatabaseConfig::default()
    })
    .await
    .expect("isolated sqlx pool should connect");
    let host = sdkwork_im_database_pool::bootstrap_im_database(pool)
        .await
        .expect("isolated IM database lifecycle should bootstrap");
    let constraints = tokio::task::spawn_blocking(move || {
        let mut client = Client::connect(&database_url, NoTls)
            .expect("isolated verification connection should connect");
        client
            .query(
                "SELECT conname, convalidated FROM pg_constraint \
                 WHERE connamespace = current_schema()::regnamespace \
                   AND conname = ANY($1) ORDER BY conname",
                &[&vec![
                    "ck_im_agent_dispatch_message_ids",
                    "ck_im_agent_dispatch_scope",
                    "ck_im_conversation_agent_binding_scope",
                    "ck_im_projection_conversation_agent_scope",
                ]],
            )
            .expect("subject guard constraints should be queryable")
            .into_iter()
            .map(|row| (row.get::<_, String>(0), row.get::<_, bool>(1)))
            .collect::<Vec<_>>()
    })
    .await
    .expect("constraint verification task should complete");

    assert_eq!(
        constraints,
        vec![
            ("ck_im_agent_dispatch_message_ids".to_string(), true),
            ("ck_im_agent_dispatch_scope".to_string(), true),
            ("ck_im_conversation_agent_binding_scope".to_string(), true,),
            (
                "ck_im_projection_conversation_agent_scope".to_string(),
                true,
            ),
        ]
    );

    drop(host);
}
