#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum ApiHttpStatus {
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
