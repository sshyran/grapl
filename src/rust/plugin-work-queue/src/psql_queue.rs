use bytes::Bytes;
use grapl_utils::future_ext::GraplFutureExt;
use sqlx::{
    Pool,
    Postgres,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    server::PluginWorkQueueInitError,
    PluginWorkQueueDbConfig,
};

#[derive(Clone, Debug, Eq, PartialEq, sqlx::Type)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Status {
    Enqueued,
    Failed,
    Processed,
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct ExecutionId(i64);

impl From<i64> for ExecutionId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<ExecutionId> for i64 {
    fn from(id: ExecutionId) -> Self {
        id.0
    }
}

#[derive(Clone, Debug, sqlx::Type)]
pub struct NextExecutionRequest {
    pub execution_key: ExecutionId,
    pub plugin_id: uuid::Uuid,
    pub pipeline_message: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum PsqlQueueError {
    #[error("Sqlx {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Clone, Debug)]
pub struct Message {
    pub request: NextExecutionRequest,
}

#[derive(Clone, Debug)]
pub struct PsqlQueue {
    pub pool: Pool<Postgres>,
}

impl PsqlQueue {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn try_from(
        service_config: PluginWorkQueueDbConfig,
    ) -> Result<Self, PluginWorkQueueInitError> {
        let postgres_address = format!(
            "postgresql://{}:{}@{}:{}",
            service_config.plugin_work_queue_db_username,
            service_config.plugin_work_queue_db_password,
            service_config.plugin_work_queue_db_hostname,
            service_config.plugin_work_queue_db_port,
        );

        let pool = sqlx::PgPool::connect(&postgres_address)
            .timeout(std::time::Duration::from_secs(5))
            .await??;

        Ok(Self::new(pool))
    }

    #[instrument(skip(self, pipeline_message), err)]
    pub async fn put_generator_message(
        &self,
        plugin_id: Uuid,
        pipeline_message: Bytes,
    ) -> Result<(), PsqlQueueError> {
        sqlx::query!(
            r"
            INSERT INTO plugin_work_queue.generator_plugin_executions (
                plugin_id,
                pipeline_message,
                current_status,
                try_count
            )
            VALUES( $1::UUID, $2, 'enqueued', -1 )
        ",
            plugin_id,
            pipeline_message.as_ref(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self, pipeline_message), err)]
    pub async fn put_analyzer_message(
        &self,
        plugin_id: Uuid,
        pipeline_message: Bytes,
    ) -> Result<(), PsqlQueueError> {
        sqlx::query!(
            r"
            INSERT INTO plugin_work_queue.analyzer_plugin_executions (
                plugin_id,
                pipeline_message,
                current_status,
                try_count
            )
            VALUES( $1::UUID, $2, 'enqueued', -1 )
        ",
            plugin_id,
            pipeline_message.as_ref(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self), err)]
    pub async fn get_generator_message(
        &self,
        plugin_id: Uuid,
    ) -> Result<Option<Message>, PsqlQueueError> {
        // This function does a few things
        // 1. It attempts to get a message from the queue
        //      -> Where that message isn't over a day old
        //      -> Where that message is "visible"
        //      -> Where that message isn't currently being evaluated by another transaction
        //      -> Where that message is in the 'enqueued' state
        // 2. Updates the `try_count`
        // 3. Updates the `visible_after`

        // Note that:
        // * messages are invisible for 10 seconds *after* each select
        //      * The 10 second timeout is arbitrary but reasonable.
        // * messages are immediately visible after their insert
        // * messages 'expire' after one day
        // * messages currently do not have a maximum retry limit
        // * The one day expiration matches our 1 day partitioning strategy

        // In the future we can leverage a maximum retry limit as well as a batch version of this query
        // A more dynamic visibility strategy would also be reasonable
        let request: Option<NextExecutionRequest> = sqlx::query_as!(
            NextExecutionRequest, r#"
            UPDATE plugin_work_queue.generator_plugin_executions
            SET
                try_count  = try_count + 1,
                last_updated = CURRENT_TIMESTAMP,
                visible_after  = CURRENT_TIMESTAMP + INTERVAL '10 seconds'
            FROM (
                SELECT execution_key, plugin_id, pipeline_message, current_status, creation_time, visible_after
                FROM plugin_work_queue.generator_plugin_executions
                WHERE
                    plugin_id = $1
                    AND current_status = 'enqueued'
                    AND creation_time >= (CURRENT_TIMESTAMP - INTERVAL '1 day')
                    AND visible_after <= CURRENT_TIMESTAMP
                ORDER BY creation_time ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            ) AS next_execution
            WHERE plugin_work_queue.generator_plugin_executions.execution_key = next_execution.execution_key
            RETURNING
                next_execution.execution_key AS "execution_key!: ExecutionId",
                next_execution.plugin_id,
                next_execution.pipeline_message
        "#,
        plugin_id
    ).fetch_optional(&self.pool)
            .await?;

        Ok(request.map(|request| Message { request }))
    }

    #[instrument(skip(self), err)]
    pub async fn get_analyzer_message(
        &self,
        plugin_id: Uuid,
    ) -> Result<Option<Message>, PsqlQueueError> {
        // `get_message` does a few things
        // 1. It attempts to get a message from the queue
        //      -> Where that message isn't over a day old
        //      -> Where that message is "visible"
        //      -> Where that message isn't currently being evaluated by another transaction
        //      -> Where that message is in the 'enqueued' state
        // 2. Updates the `try_count`
        // 3. Updates the `visible_after`

        // Note that:
        // * messages are invisible for 10 seconds *after* each select
        //      * The 10 second timeout is arbitrary but reasonable.
        // * messages are immediately visible after their insert
        // * messages 'expire' after one day
        // * messages currently do not have a maximum retry limit
        // * The one day expiration matches our 1 day partitioning strategy

        // In the future we can leverage a maximum retry limit as well as a batch version of this query
        // A more dynamic visibility strategy would also be reasonable
        let request: Option<NextExecutionRequest> = sqlx::query_as!(
            NextExecutionRequest, r#"
            UPDATE plugin_work_queue.analyzer_plugin_executions
            SET
                try_count  = plugin_work_queue.analyzer_plugin_executions.try_count + 1,
                last_updated = CURRENT_TIMESTAMP,
                visible_after  = CURRENT_TIMESTAMP + INTERVAL '10 seconds'
            FROM (
                SELECT execution_key, plugin_id, pipeline_message, current_status, creation_time, visible_after
                FROM plugin_work_queue.analyzer_plugin_executions
                WHERE 
                    plugin_id = $1
                    AND current_status = 'enqueued'
                    AND creation_time >= (CURRENT_TIMESTAMP - INTERVAL '1 day')
                    AND visible_after <= CURRENT_TIMESTAMP
                ORDER BY creation_time ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            ) AS next_execution
            WHERE plugin_work_queue.analyzer_plugin_executions.execution_key = next_execution.execution_key
            RETURNING
                next_execution.execution_key AS "execution_key!: ExecutionId",
                next_execution.plugin_id,
                next_execution.pipeline_message
        "#, plugin_id).fetch_optional(&self.pool)
            .await?;

        Ok(request.map(|request| Message { request }))
    }

    #[instrument(skip(self), err)]
    pub async fn ack_generator(
        &self,
        execution_key: ExecutionId,
        current_status: Status,
    ) -> Result<(), PsqlQueueError> {
        sqlx::query!(
            r#"
                UPDATE plugin_work_queue.generator_plugin_executions
                SET current_status = $2,
                    last_updated = CASE
                        WHEN current_status != 'processed'
                            THEN CURRENT_TIMESTAMP
                            ELSE last_updated
                        END
                WHERE execution_key = $1
            "#,
            execution_key.0,
            current_status as _,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self), err)]
    pub async fn ack_analyzer(
        &self,
        execution_key: ExecutionId,
        current_status: Status,
    ) -> Result<(), PsqlQueueError> {
        sqlx::query!(
            r#"
                UPDATE plugin_work_queue.analyzer_plugin_executions
                SET current_status = $2,
                    last_updated = CASE
                        WHEN current_status != 'processed'
                            THEN CURRENT_TIMESTAMP
                            ELSE last_updated
                        END
                WHERE execution_key = $1
            "#,
            execution_key.0,
            current_status as _,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
