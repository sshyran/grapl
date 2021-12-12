use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};
use sqlx::types::Uuid;
use crate::orgmanagementlib::organization_manager_server::OrganizationManager;
use crate::orgmanagementlib::{
    CreateOrgRequest,
    CreateUserRequest,
    EmptyResp,
    // UserRequest,
};

#[derive(Debug)]
pub struct OrganizationManagerRpc {
    pub pool: Pool<Postgres>,
}

#[derive(thiserror::Error, Debug)]
pub enum OrganizationManagerError {
    #[error("sql")]
    Sql(#[from] sqlx::Error),
}

impl From<OrganizationManagerError> for Status {
    fn from(e: OrganizationManagerError) -> Self {
        match e {
            OrganizationManagerError::Sql(e) => Status::internal(e.to_string()),
        }
    }
}

#[tonic::async_trait]
impl OrganizationManager for OrganizationManagerRpc {

    async fn create_org(
        &self,
        request: Request<CreateOrgRequest>,
    ) -> Result<Response<EmptyResp>, Status> {
        println!("Org request data: {:?}", &request);

        let org_id = sqlx::types::Uuid::from_u128(Uuid::new_v4().as_u128());

        let CreateOrgRequest {
            org_display_name,
            admin_username,
            admin_email,
            admin_password,
            should_reset_password,
        } = &request.into_inner();

        let row = sqlx::query!(
            r"
            INSERT INTO organization (
                org_id,
                org_display_name,
                admin_username,
                admin_email,
                admin_password,
                should_reset_password
            )
             VALUES ( $1, $2, $3, $4, $5, $6 )
        ",
        org_id,
        org_display_name,
        admin_username,
        admin_email,
        admin_password,
        should_reset_password
        ).execute(&self.pool)
            .await
            .map_err(OrganizationManagerError::from)?;

        if row.rows_affected() == 0 {
            return Err(Status::internal(
                "Organization was not created successfully",
            ));
        }

        Ok(Response::new(EmptyResp {}))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<EmptyResp>, Status> {
        println!("Org request data: {:?}", &request); // don't actually print this

        let user_id = sqlx::types::Uuid::from_u128(Uuid::new_v4().as_u128());
        let org_id = sqlx::types::Uuid::from_u128(Uuid::new_v4().as_u128());

        let CreateUserRequest {
            // org_id, // we need to do a lookup here
            name,
            email,
            password,
        } = &request.into_inner();

        let row = sqlx::query!(
            r"
            INSERT INTO users (
                user_id,
                org_id,
                name,
                email,
                password
            )
             VALUES ( $1, $2, $3, $4, $5 )
        ",
            user_id,
            org_id,
            name,
            email,
            password,
        ).execute(&self.pool)
            .await
            .map_err(OrganizationManagerError::from)?;

        if row.rows_affected() == 0 {
            return Err(Status::internal("User was not created successfully"));
        }

        Ok(Response::new(EmptyResp {}))
    }

    //
    // async fn delete_user(
    //     &self,
    //     request: Request<UserRequest>,
    // ) ->  Result<Response<EmptyResp>, Status> {
    //
    //     let UserRequest {
    //         user_id
    //     } = request.into_inner();
    //
    //
    //     let row = sqlx::query!(
    //         r"
    //             DELETE FROM users WHERE user_id = $1
    //         ",
    //         user_id,
    //     ).execute(&self.pool)
    //         .await
    //         .map_err(OrganizationManagerError::from)?;
    //
    //     // let user_id = sqlx::types::Uuid::from_u128(Uuid::new_v4().as_u128());
    //
    //     if row.rows_affected() == 0 {
    //         return Err(Status::internal(
    //             "Unable to delete user",
    //         ));
    //     }
    //
    //     Ok(Response::new(EmptyResp {}))
    //
    // }
}