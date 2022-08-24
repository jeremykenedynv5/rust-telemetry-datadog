// Copyright © 2022 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    tracing::info!("Successfully able to ping the application with health_check api!");
    HttpResponse::Ok().finish()
}
