use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::services::auth::{decode_token, Claims};

pub async fn auth_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {

    let token = match jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request
                .headers()
                .get("authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(|s| s.to_string())
        }) {
        Some(t) => t,
        None => {
            return unauthorized_response("No token provided");
        }
    };

    let claims = match decode_token(&token) {
        Ok(c) => c,
        Err(_) => {
            return unauthorized_response("Invalid token");
        }
    };

    if claims.token_type != "access" {
        return unauthorized_response("Invalid token type");
    }

    request.extensions_mut().insert(claims);

    next.run(request).await
}

pub async fn optional_auth_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(token) = jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request
                .headers()
                .get("authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(|s| s.to_string())
        })
    {
        if let Ok(claims) = decode_token(&token) {
            if claims.token_type == "access" {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}

fn unauthorized_response(message: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": message })),
    )
        .into_response()
}

fn forbidden_response(message: &str) -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(json!({ "error": message })),
    )
        .into_response()
}

pub async fn require_staff_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request
                .headers()
                .get("authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(|s| s.to_string())
        }) {
        Some(t) => t,
        None => {
            return forbidden_response("Access denied");
        }
    };

    let claims = match decode_token(&token) {
        Ok(c) => c,
        Err(_) => {
            return forbidden_response("Access denied");
        }
    };

    if claims.token_type != "access" {
        return forbidden_response("Access denied");
    }

    if !claims.is_staff && !claims.is_superuser {
        return forbidden_response("Access denied");
    }

    request.extensions_mut().insert(claims);

    next.run(request).await
}

pub async fn require_superuser_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            request
                .headers()
                .get("authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .map(|s| s.to_string())
        }) {
        Some(t) => t,
        None => {
            return forbidden_response("Superuser access required");
        }
    };

    let claims = match decode_token(&token) {
        Ok(c) => c,
        Err(_) => {
            return forbidden_response("Superuser access required");
        }
    };

    if claims.token_type != "access" {
        return forbidden_response("Superuser access required");
    }

    if !claims.is_superuser {
        return forbidden_response("Superuser access required");
    }

    request.extensions_mut().insert(claims);

    next.run(request).await
}
