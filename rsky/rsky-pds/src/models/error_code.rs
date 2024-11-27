use anyhow::{bail, Result};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl ErrorCode {
    pub fn from_str(code: &str) -> Result<Self> {
        match code {
            "MultipleChoices" => Ok(Self::MultipleChoices),
            "MovedPermanently" => Ok(Self::MovedPermanently),
            "Found" => Ok(Self::Found),
            "SeeOther" => Ok(Self::SeeOther),
            "NotModified" => Ok(Self::NotModified),
            "UseProxy" => Ok(Self::UseProxy),
            "TemporaryRedirect" => Ok(Self::TemporaryRedirect),
            "PermanentRedirect" => Ok(Self::PermanentRedirect),
            "BadRequest" => Ok(Self::BadRequest),
            "Unauthorized" => Ok(Self::Unauthorized),
            "PaymentRequired" => Ok(Self::PaymentRequired),
            "Forbidden" => Ok(Self::Forbidden),
            "NotFound" => Ok(Self::NotFound),
            "MethodNotAllowed" => Ok(Self::MethodNotAllowed),
            "NotAcceptable" => Ok(Self::NotAcceptable),
            "ProxyAuthenticationRequired" => Ok(Self::ProxyAuthenticationRequired),
            "RequestTimeout" => Ok(Self::RequestTimeout),
            "Conflict" => Ok(Self::Conflict),
            "Gone" => Ok(Self::Gone),
            "LengthRequired" => Ok(Self::LengthRequired),
            "PreconditionFailed" => Ok(Self::PreconditionFailed),
            "PayloadTooLarge" => Ok(Self::PayloadTooLarge),
            "UriTooLong" => Ok(Self::UriTooLong),
            "UnsupportedMediaType" => Ok(Self::UnsupportedMediaType),
            "RangeNotSatisfiable" => Ok(Self::RangeNotSatisfiable),
            "ExpectationFailed" => Ok(Self::ExpectationFailed),
            "ImATeapot" => Ok(Self::ImATeapot),
            "MisdirectedRequest" => Ok(Self::MisdirectedRequest),
            "UnprocessableEntity" => Ok(Self::UnprocessableEntity),
            "Locked" => Ok(Self::Locked),
            "FailedDependency" => Ok(Self::FailedDependency),
            "UpgradeRequired" => Ok(Self::UpgradeRequired),
            "PreconditionRequired" => Ok(Self::PreconditionRequired),
            "TooManyRequests" => Ok(Self::TooManyRequests),
            "RequestHeaderFieldsTooLarge" => Ok(Self::RequestHeaderFieldsTooLarge),
            "UnavailableForLegalReasons" => Ok(Self::UnavailableForLegalReasons),
            "InternalServerError" => Ok(Self::InternalServerError),
            "NotImplemented" => Ok(Self::NotImplemented),
            "BadGateway" => Ok(Self::BadGateway),
            "ServiceUnavailable" => Ok(Self::ServiceUnavailable),
            "GatewayTimeout" => Ok(Self::GatewayTimeout),
            "HttpVersionNotSupported" => Ok(Self::HttpVersionNotSupported),
            "VariantAlsoNegotiates" => Ok(Self::VariantAlsoNegotiates),
            "InsufficientStorage" => Ok(Self::InsufficientStorage),
            "LoopDetected" => Ok(Self::LoopDetected),
            "NotExtended" => Ok(Self::NotExtended),
            "NetworkAuthenticationRequired" => Ok(Self::NetworkAuthenticationRequired),
            _ => bail!("Invalid ErrorCode: `{code:?}` is not a valid error code"),
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::MultipleChoices => String::from("MultipleChoices"),
            Self::MovedPermanently => String::from("MovedPermanently"),
            Self::Found => String::from("Found"),
            Self::SeeOther => String::from("SeeOther"),
            Self::NotModified => String::from("NotModified"),
            Self::UseProxy => String::from("UseProxy"),
            Self::TemporaryRedirect => String::from("TemporaryRedirect"),
            Self::PermanentRedirect => String::from("PermanentRedirect"),
            Self::BadRequest => String::from("BadRequest"),
            Self::Unauthorized => String::from("Unauthorized"),
            Self::PaymentRequired => String::from("PaymentRequired"),
            Self::Forbidden => String::from("Forbidden"),
            Self::NotFound => String::from("NotFound"),
            Self::MethodNotAllowed => String::from("MethodNotAllowed"),
            Self::NotAcceptable => String::from("NotAcceptable"),
            Self::ProxyAuthenticationRequired => String::from("ProxyAuthenticationRequired"),
            Self::RequestTimeout => String::from("RequestTimeout"),
            Self::Conflict => String::from("Conflict"),
            Self::Gone => String::from("Gone"),
            Self::LengthRequired => String::from("LengthRequired"),
            Self::PreconditionFailed => String::from("PreconditionFailed"),
            Self::PayloadTooLarge => String::from("PayloadTooLarge"),
            Self::UriTooLong => String::from("UriTooLong"),
            Self::UnsupportedMediaType => String::from("UnsupportedMediaType"),
            Self::RangeNotSatisfiable => String::from("RangeNotSatisfiable"),
            Self::ExpectationFailed => String::from("ExpectationFailed"),
            Self::ImATeapot => String::from("ImATeapot"),
            Self::MisdirectedRequest => String::from("MisdirectedRequest"),
            Self::UnprocessableEntity => String::from("UnprocessableEntity"),
            Self::Locked => String::from("Locked"),
            Self::FailedDependency => String::from("FailedDependency"),
            Self::UpgradeRequired => String::from("UpgradeRequired"),
            Self::PreconditionRequired => String::from("PreconditionRequired"),
            Self::TooManyRequests => String::from("TooManyRequests"),
            Self::RequestHeaderFieldsTooLarge => String::from("RequestHeaderFieldsTooLarge"),
            Self::UnavailableForLegalReasons => String::from("UnavailableForLegalReasons"),
            Self::InternalServerError => String::from("InternalServerError"),
            Self::NotImplemented => String::from("NotImplemented"),
            Self::BadGateway => String::from("BadGateway"),
            Self::ServiceUnavailable => String::from("ServiceUnavailable"),
            Self::GatewayTimeout => String::from("GatewayTimeout"),
            Self::HttpVersionNotSupported => String::from("HttpVersionNotSupported"),
            Self::VariantAlsoNegotiates => String::from("VariantAlsoNegotiates"),
            Self::InsufficientStorage => String::from("InsufficientStorage"),
            Self::LoopDetected => String::from("LoopDetected"),
            Self::NotExtended => String::from("NotExtended"),
            Self::NetworkAuthenticationRequired => String::from("NetworkAuthenticationRequired"),
        };
        write!(f, "{}", str)
    }
}
