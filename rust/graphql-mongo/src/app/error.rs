use async_graphql::{Error, ErrorExtensions};

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum ApiHttpError {
    // Informational
    #[error("Continue")]
    Continue(String),

    #[error("SwitchingProtocols")]
    SwitchingProtocols(String),

    #[error("Processing")]
    Processing(String),

    // Success
    #[error("OK")]
    OK(String),

    #[error("Created")]
    Created(String),

    #[error("Accepted")]
    Accepted(String),

    #[error("NonauthoritativeInformation")]
    NonauthoritativeInformation(String),

    #[error("NoContent")]
    NoContent(String),

    #[error("ResetContent")]
    ResetContent(String),

    #[error("PartialContent")]
    PartialContent(String),

    #[error("MultiStatus")]
    MultiStatus(String),

    #[error("AlreadyReported")]
    AlreadyReported(String),

    #[error("ImUsed")]
    ImUsed(String),

    // Redirection
    #[error("MultipleChoices")]
    MultipleChoices(String),

    #[error("MovedPermanently")]
    MovedPermanently(String),

    #[error("Found")]
    Found(String),

    #[error("SeeOther")]
    SeeOther(String),

    #[error("NotModified")]
    NotModified(String),

    #[error("UseProxy")]
    UseProxy(String),

    #[error("TemporaryRedirect")]
    TemporaryRedirect(String),

    #[error("PermanentRedirect")]
    PermanentRedirect(String),

    // ClientError
    #[error("BadRequest")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized(String),

    #[error("PaymentRequired")]
    PaymentRequired(String),

    #[error("Forbidden")]
    Forbidden(String),

    #[error("NotFound")]
    NotFound(String),

    #[error("MethodNotAllowed")]
    MethodNotAllowed(String),

    #[error("NotAcceptable")]
    NotAcceptable(String),

    #[error("ProxyAuthenticationRequired")]
    ProxyAuthenticationRequired(String),

    #[error("RequestTimeout")]
    RequestTimeout(String),

    #[error("Conflict")]
    Conflict(String),

    #[error("Gone")]
    Gone(String),

    #[error("LengthRequired")]
    LengthRequired(String),

    #[error("PreconditionFailed")]
    PreconditionFailed(String),

    #[error("PayloadTooLarge")]
    PayloadTooLarge(String),

    #[error("RequestURITooLong")]
    RequestURITooLong(String),

    #[error("UnsupportedMediaType")]
    UnsupportedMediaType(String),

    #[error("RequestedRangeNotSatisfiable")]
    RequestedRangeNotSatisfiable(String),

    #[error("ExpectationFailed")]
    ExpectationFailed(String),

    #[error("ImATeapot")]
    ImATeapot(String),

    #[error("MisdirectedRequest")]
    MisdirectedRequest(String),

    #[error("UnprocessableEntity")]
    UnprocessableEntity(String),

    #[error("Locked")]
    Locked(String),

    #[error("FailedDependency")]
    FailedDependency(String),

    #[error("UpgradeRequired")]
    UpgradeRequired(String),

    #[error("PreconditionRequired")]
    PreconditionRequired(String),

    #[error("TooManyRequests")]
    TooManyRequests(String),

    #[error("RequestHeaderFieldsTooLarge")]
    RequestHeaderFieldsTooLarge(String),

    #[error("ConnectionClosedWithoutResponse")]
    ConnectionClosedWithoutResponse(String),

    #[error("UnavailableForLegalReasons")]
    UnavailableForLegalReasons(String),

    #[error("ClientClosedRequest")]
    ClientClosedRequest(String),

    // ServerError
    #[error("InternalServerError")]
    InternalServerError(String),

    #[error("NotImplemented")]
    NotImplemented(String),

    #[error("BadGateway")]
    BadGateway(String),

    #[error("ServiceUnavailable")]
    ServiceUnavailable(String),

    #[error("GatewayTimeout")]
    GatewayTimeout(String),

    #[error("HTTPVersionNotSupported")]
    HTTPVersionNotSupported(String),

    #[error("VariantAlsoNegotiates")]
    VariantAlsoNegotiates(String),

    #[error("InsufficientStorage")]
    InsufficientStorage(String),

    #[error("LoopDetected")]
    LoopDetected(String),

    #[error("NotExtended")]
    NotExtended(String),

    #[error("NetworkAuthenticationRequired")]
    NetworkAuthenticationRequired(String),

    #[error("NetworkConnectTimeoutError")]
    NetworkConnectTimeoutError(String),
}

impl ErrorExtensions for ApiHttpError {
    // lets define our base extensions
    fn extend(&self) -> Error {
        use ApiHttpError::*;
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            Continue(details) => {
                e.set("code", format!("100"));
                e.set("details", details.as_str());
            }
            SwitchingProtocols(details) => {
                e.set("code", format!("101"));
                e.set("details", details.as_str());
            }
            Processing(details) => {
                e.set("code", format!("102"));
                e.set("details", details.as_str());
            }
            OK(details) => {
                e.set("code", format!("200"));
                e.set("details", details.as_str());
            }
            Created(details) => {
                e.set("code", format!("201"));
                e.set("details", details.as_str());
            }
            Accepted(details) => {
                e.set("code", format!("202"));
                e.set("details", details.as_str());
            }
            NonauthoritativeInformation(details) => {
                e.set("code", format!("203"));
                e.set("details", details.as_str());
            }
            NoContent(details) => {
                e.set("code", format!("204"));
                e.set("details", details.as_str());
            }
            ResetContent(details) => {
                e.set("code", format!("205"));
                e.set("details", details.as_str());
            }
            PartialContent(details) => {
                e.set("code", format!("206"));
                e.set("details", details.as_str());
            }
            MultiStatus(details) => {
                e.set("code", format!("207"));
                e.set("details", details.as_str());
            }
            AlreadyReported(details) => {
                e.set("code", format!("208"));
                e.set("details", details.as_str());
            }
            ImUsed(details) => {
                e.set("code", format!("226"));
                e.set("details", details.as_str());
            }

            MultipleChoices(details) => {
                e.set("code", format!("300"));
                e.set("details", details.as_str());
            }
            MovedPermanently(details) => {
                e.set("code", format!("301"));
                e.set("details", details.as_str());
            }
            Found(details) => {
                e.set("code", format!("302"));
                e.set("details", details.as_str());
            }
            SeeOther(details) => {
                e.set("code", format!("303"));
                e.set("details", details.as_str());
            }
            NotModified(details) => {
                e.set("code", format!("304"));
                e.set("details", details.as_str());
            }
            UseProxy(details) => {
                e.set("code", format!("305"));
                e.set("details", details.as_str());
            }
            TemporaryRedirect(details) => {
                e.set("code", format!("307"));
                e.set("details", details.as_str());
            }
            PermanentRedirect(details) => {
                e.set("code", format!("308"));
                e.set("details", details.as_str());
            }
            BadRequest(details) => {
                e.set("code", format!("400"));
                e.set("details", details.as_str());
            }
            Unauthorized(details) => {
                e.set("code", format!("401"));
                e.set("details", details.as_str());
            }
            PaymentRequired(details) => {
                e.set("code", format!("402"));
                e.set("details", details.as_str());
            }
            Forbidden(details) => {
                e.set("code", format!("403"));
                e.set("details", details.as_str());
            }
            NotFound(details) => {
                e.set("code", format!("404"));
                e.set("details", details.as_str());
            }
            MethodNotAllowed(details) => {
                e.set("code", format!("405"));
                e.set("details", details.as_str());
            }
            NotAcceptable(details) => {
                e.set("code", format!("406"));
                e.set("details", details.as_str());
            }
            ProxyAuthenticationRequired(details) => {
                e.set("code", format!("407"));
                e.set("details", details.as_str());
            }
            RequestTimeout(details) => {
                e.set("code", format!("408"));
                e.set("details", details.as_str());
            }
            Conflict(details) => {
                e.set("code", format!("409"));
                e.set("details", details.as_str());
            }
            Gone(details) => {
                e.set("code", format!("410"));
                e.set("details", details.as_str());
            }
            LengthRequired(details) => {
                e.set("code", format!("411"));
                e.set("details", details.as_str());
            }
            PreconditionFailed(details) => {
                e.set("code", format!("412"));
                e.set("details", details.as_str());
            }
            PayloadTooLarge(details) => {
                e.set("code", format!("413"));
                e.set("details", details.as_str());
            }
            RequestURITooLong(details) => {
                e.set("code", format!("414"));
                e.set("details", details.as_str());
            }
            UnsupportedMediaType(details) => {
                e.set("code", format!("415"));
                e.set("details", details.as_str());
            }
            RequestedRangeNotSatisfiable(details) => {
                e.set("code", format!("416"));
                e.set("details", details.as_str());
            }
            ExpectationFailed(details) => {
                e.set("code", format!("417"));
                e.set("details", details.as_str());
            }
            ImATeapot(details) => {
                e.set("code", format!("418"));
                e.set("details", details.as_str());
            }
            MisdirectedRequest(details) => {
                e.set("code", format!("421"));
                e.set("details", details.as_str());
            }
            UnprocessableEntity(details) => {
                e.set("code", format!("422"));
                e.set("details", details.as_str());
            }
            Locked(details) => {
                e.set("code", format!("423"));
                e.set("details", details.as_str());
            }
            FailedDependency(details) => {
                e.set("code", format!("424"));
                e.set("details", details.as_str());
            }
            UpgradeRequired(details) => {
                e.set("code", format!("426"));
                e.set("details", details.as_str());
            }
            PreconditionRequired(details) => {
                e.set("code", format!("428"));
                e.set("details", details.as_str());
            }
            TooManyRequests(details) => {
                e.set("code", format!("429"));
                e.set("details", details.as_str());
            }
            RequestHeaderFieldsTooLarge(details) => {
                e.set("code", format!("431"));
                e.set("details", details.as_str());
            }
            ConnectionClosedWithoutResponse(details) => {
                e.set("code", format!("444"));
                e.set("details", details.as_str());
            }
            UnavailableForLegalReasons(details) => {
                e.set("code", format!("451"));
                e.set("details", details.as_str());
            }
            ClientClosedRequest(details) => {
                e.set("code", format!("499"));
                e.set("details", details.as_str());
            }

            InternalServerError(details) => {
                e.set("code", format!("500"));
                e.set("details", details.as_str());
            }
            NotImplemented(details) => {
                e.set("code", format!("501"));
                e.set("details", details.as_str());
            }
            BadGateway(details) => {
                e.set("code", format!("502"));
                e.set("details", details.as_str());
            }
            ServiceUnavailable(details) => {
                e.set("code", format!("503"));
                e.set("details", details.as_str());
            }
            GatewayTimeout(details) => {
                e.set("code", format!("504"));
                e.set("details", details.as_str());
            }
            HTTPVersionNotSupported(details) => {
                e.set("code", format!("505"));
                e.set("details", details.as_str());
            }
            VariantAlsoNegotiates(details) => {
                e.set("code", format!("506"));
                e.set("details", details.as_str());
            }
            InsufficientStorage(details) => {
                e.set("code", format!("507"));
                e.set("details", details.as_str());
            }
            LoopDetected(details) => {
                e.set("code", format!("508"));
                e.set("details", details.as_str());
            }
            NotExtended(details) => {
                e.set("code", format!("510"));
                e.set("details", details.as_str());
            }
            NetworkAuthenticationRequired(details) => {
                e.set("code", format!("511"));
                e.set("details", details.as_str());
            }
            NetworkConnectTimeoutError(details) => {
                e.set("code", format!("599"));
                e.set("details", details.as_str());
            }
        })
    }
}
