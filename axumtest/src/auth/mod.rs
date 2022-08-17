//! "Authentication" middleware.

use axum::headers::HeaderName;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

static CIUSR: HeaderName = HeaderName::from_static("x-ciusr");
static CIPWD: HeaderName = HeaderName::from_static("x-cipwd");
static CIROLE: HeaderName = HeaderName::from_static("x-cirole");

pub async fn ci_auth<B>(
    req: Request<B>,
    next: Next<B>,
    expected_usr: String,
    expected_pwd: String,
    expected_role: String,
) -> Result<Response, StatusCode> {
    let usr = req
        .headers()
        .get(&CIUSR)
        .and_then(|header| header.to_str().ok());
    let pwd = req
        .headers()
        .get(&CIPWD)
        .and_then(|header| header.to_str().ok());
    let role = req
        .headers()
        .get(&CIROLE)
        .and_then(|header| header.to_str().ok());

    tracing::debug!(usr, pwd, role);

    match (usr, pwd, role) {
        (Some(inc_usr), Some(inc_pwd), Some(inc_role))
            if inc_usr == expected_usr && inc_pwd == expected_pwd && inc_role == expected_role =>
        {
            Ok(next.run(req).await)
        }
        (_, _, _) => Err(StatusCode::UNAUTHORIZED),
    }
}
