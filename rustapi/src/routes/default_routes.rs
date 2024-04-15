use axum::Extension;
use axum::response::IntoResponse;
use axum_macros::debug_handler;
use crate::types::data_representations::GoogleUser;

#[debug_handler]
pub(crate) async fn root(Extension(user): Extension<GoogleUser>) -> impl IntoResponse
{
   format!(
            "Hey {}! You're logged in!\n\
            You may now access `/protected`.\n\
            Log out with `/logout`.",
            user.name
   )
    .into_response()
}

