#![cfg(all(test, feature = "integration"))]

use rust_proto_new::graplinc::grapl::api::uid_allocator::v1beta1::client::UidAllocatorClient;
use sqlx::PgPool;
use structopt::StructOpt;
use uid_allocator::client::CachingUidAllocatorClient;

#[tokio::test]
/// This test allocates 3 times. After the first two allocations, the preallocated space
/// is exhausted.
async fn test_uid_allocator() -> Result<(), Box<dyn std::error::Error>> {
    let counter_db_config = uid_allocator::config::CounterDbConfig::from_args();
    let client_config = uid_allocator::config::UidAllocatorClientConfig::from_args();

    let pool = PgPool::connect(&counter_db_config.to_postgres_url()).await?;

    sqlx::migrate!().run(&pool).await?;

    let tenant_id = uuid::Uuid::new_v4();

    sqlx::query("INSERT INTO counters (tenant_id) VALUES ($1);")
        .bind(tenant_id)
        .execute(&pool)
        .await?;

    let endpoint = format!("http://{}", &client_config.uid_allocator_connect_address);
    let mut allocator_client =
        CachingUidAllocatorClient::new(UidAllocatorClient::connect(endpoint).await?, 100);

    for i in 1u64..1000 {
        let next_id = allocator_client.allocate_id(tenant_id).await?;
        assert_eq!(next_id, i);
    }

    Ok(())
}
