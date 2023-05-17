use axum::{extract::State, routing::delete, Router};

use crate::{
    prisma::user::{self, Data},
    user::request::{delete::DeleteUserRequest, loggedin::LoggedInUser},
    validation::validation_message,
    web::Web,
    Database, WebResult,
};

use super::UserController;

impl UserController {
    pub fn delete_user(&self) -> Router<Database> {
        async fn delete_user_handler(
            State(db): State<Database>,
            LoggedInUser(Data {
                id,
                password: user_password,
                ..
            }): LoggedInUser,
            DeleteUserRequest { password, .. }: DeleteUserRequest,
        ) -> WebResult {
            match user_password == password {
                true => {
                    db.user().delete(user::id::equals(id)).exec().await?;
                    Ok(Web::ok("Deleted user successfully", ()))
                }
                false => Err(validation_message(
                    "Password provided does not match current password",
                )
                .into()),
            }
        }
        Router::new().route("/delete", delete(delete_user_handler))
    }
}
