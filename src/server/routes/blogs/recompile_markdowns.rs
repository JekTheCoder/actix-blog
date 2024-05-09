use actix_web::{post, HttpResponse};

use crate::{
    domain::blog::features::recompile_markdowns::RecompileMarkdowns, server::admin::IsAdminFactory,
};

#[post("/recompile-markdowns/", wrap = "IsAdminFactory")]
pub async fn endpoint(recompile: RecompileMarkdowns) -> HttpResponse {
    match recompile.run().await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
