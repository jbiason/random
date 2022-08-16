//! Header parsing for X-CIROLE

use axum::headers::Header;
use axum::headers::HeaderName;
use axum::headers::HeaderValue;

static NAME: HeaderName = HeaderName::from_static("x-cirole");

pub struct CiRole(String);

impl Header for CiRole {
    fn name() -> &'static HeaderName {
        &NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let value = values.next().ok_or_else(axum::headers::Error::invalid)?;
        Ok(CiRole(
            value
                .to_str()
                .or(Err(axum::headers::Error::invalid()))?
                .into(),
        ))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(HeaderValue::from_str(&self.0).unwrap()))
    }
}

impl std::fmt::Display for CiRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
