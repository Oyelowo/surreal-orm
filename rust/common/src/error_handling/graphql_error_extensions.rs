use super::api_http_statuses::ApiHttpStatus;
use async_graphql::{Error, ErrorExtensions};

impl ErrorExtensions for ApiHttpStatus {
    // lets define our base extensions
    fn extend(&self) -> Error {
        use ApiHttpStatus::*;
        Error::new(format!("{self}",)).extend_with(|_err, e| match self {
            Continue(details) => {
                e.set("code", "100".to_string());
                e.set("details", details.as_str());
            }
            SwitchingProtocols(details) => {
                e.set("code", "101".to_string());
                e.set("details", details.as_str());
            }
            Processing(details) => {
                e.set("code", "102".to_string());
                e.set("details", details.as_str());
            }
            OK(details) => {
                e.set("code", "200".to_string());
                e.set("details", details.as_str());
            }
            Created(details) => {
                e.set("code", "201".to_string());
                e.set("details", details.as_str());
            }
            Accepted(details) => {
                e.set("code", "202".to_string());
                e.set("details", details.as_str());
            }
            NonauthoritativeInformation(details) => {
                e.set("code", "203".to_string());
                e.set("details", details.as_str());
            }
            NoContent(details) => {
                e.set("code", "204".to_string());
                e.set("details", details.as_str());
            }
            ResetContent(details) => {
                e.set("code", "205".to_string());
                e.set("details", details.as_str());
            }
            PartialContent(details) => {
                e.set("code", "206".to_string());
                e.set("details", details.as_str());
            }
            MultiStatus(details) => {
                e.set("code", "207".to_string());
                e.set("details", details.as_str());
            }
            AlreadyReported(details) => {
                e.set("code", "208".to_string());
                e.set("details", details.as_str());
            }
            ImUsed(details) => {
                e.set("code", "226".to_string());
                e.set("details", details.as_str());
            }

            MultipleChoices(details) => {
                e.set("code", "300".to_string());
                e.set("details", details.as_str());
            }
            MovedPermanently(details) => {
                e.set("code", "301".to_string());
                e.set("details", details.as_str());
            }
            Found(details) => {
                e.set("code", "302".to_string());
                e.set("details", details.as_str());
            }
            SeeOther(details) => {
                e.set("code", "303".to_string());
                e.set("details", details.as_str());
            }
            NotModified(details) => {
                e.set("code", "304".to_string());
                e.set("details", details.as_str());
            }
            UseProxy(details) => {
                e.set("code", "305".to_string());
                e.set("details", details.as_str());
            }
            TemporaryRedirect(details) => {
                e.set("code", "307".to_string());
                e.set("details", details.as_str());
            }
            PermanentRedirect(details) => {
                e.set("code", "308".to_string());
                e.set("details", details.as_str());
            }
            BadRequest(details) => {
                e.set("code", "400".to_string());
                e.set("details", details.as_str());
            }
            Unauthorized(details) => {
                e.set("code", "401".to_string());
                e.set("details", details.as_str());
            }
            PaymentRequired(details) => {
                e.set("code", "402".to_string());
                e.set("details", details.as_str());
            }
            Forbidden(details) => {
                e.set("code", "403".to_string());
                e.set("details", details.as_str());
            }
            NotFound(details) => {
                e.set("code", "404".to_string());
                e.set("details", details.as_str());
            }
            MethodNotAllowed(details) => {
                e.set("code", "405".to_string());
                e.set("details", details.as_str());
            }
            NotAcceptable(details) => {
                e.set("code", "406".to_string());
                e.set("details", details.as_str());
            }
            ProxyAuthenticationRequired(details) => {
                e.set("code", "407".to_string());
                e.set("details", details.as_str());
            }
            RequestTimeout(details) => {
                e.set("code", "408".to_string());
                e.set("details", details.as_str());
            }
            Conflict(details) => {
                e.set("code", "409".to_string());
                e.set("details", details.as_str());
            }
            Gone(details) => {
                e.set("code", "410".to_string());
                e.set("details", details.as_str());
            }
            LengthRequired(details) => {
                e.set("code", "411".to_string());
                e.set("details", details.as_str());
            }
            PreconditionFailed(details) => {
                e.set("code", "412".to_string());
                e.set("details", details.as_str());
            }
            PayloadTooLarge(details) => {
                e.set("code", "413".to_string());
                e.set("details", details.as_str());
            }
            RequestURITooLong(details) => {
                e.set("code", "414".to_string());
                e.set("details", details.as_str());
            }
            UnsupportedMediaType(details) => {
                e.set("code", "415".to_string());
                e.set("details", details.as_str());
            }
            RequestedRangeNotSatisfiable(details) => {
                e.set("code", "416".to_string());
                e.set("details", details.as_str());
            }
            ExpectationFailed(details) => {
                e.set("code", "417".to_string());
                e.set("details", details.as_str());
            }
            ImATeapot(details) => {
                e.set("code", "418".to_string());
                e.set("details", details.as_str());
            }
            MisdirectedRequest(details) => {
                e.set("code", "421".to_string());
                e.set("details", details.as_str());
            }
            UnprocessableEntity(details) => {
                e.set("code", "422".to_string());
                e.set("details", details.as_str());
            }
            Locked(details) => {
                e.set("code", "423".to_string());
                e.set("details", details.as_str());
            }
            FailedDependency(details) => {
                e.set("code", "424".to_string());
                e.set("details", details.as_str());
            }
            UpgradeRequired(details) => {
                e.set("code", "426".to_string());
                e.set("details", details.as_str());
            }
            PreconditionRequired(details) => {
                e.set("code", "428".to_string());
                e.set("details", details.as_str());
            }
            TooManyRequests(details) => {
                e.set("code", "429".to_string());
                e.set("details", details.as_str());
            }
            RequestHeaderFieldsTooLarge(details) => {
                e.set("code", "431".to_string());
                e.set("details", details.as_str());
            }
            ConnectionClosedWithoutResponse(details) => {
                e.set("code", "444".to_string());
                e.set("details", details.as_str());
            }
            UnavailableForLegalReasons(details) => {
                e.set("code", "451".to_string());
                e.set("details", details.as_str());
            }
            ClientClosedRequest(details) => {
                e.set("code", "499".to_string());
                e.set("details", details.as_str());
            }

            InternalServerError(details) => {
                e.set("code", "500".to_string());
                e.set("details", details.as_str());
            }
            NotImplemented(details) => {
                e.set("code", "501".to_string());
                e.set("details", details.as_str());
            }
            BadGateway(details) => {
                e.set("code", "502".to_string());
                e.set("details", details.as_str());
            }
            ServiceUnavailable(details) => {
                e.set("code", "503".to_string());
                e.set("details", details.as_str());
            }
            GatewayTimeout(details) => {
                e.set("code", "504".to_string());
                e.set("details", details.as_str());
            }
            HTTPVersionNotSupported(details) => {
                e.set("code", "505".to_string());
                e.set("details", details.as_str());
            }
            VariantAlsoNegotiates(details) => {
                e.set("code", "506".to_string());
                e.set("details", details.as_str());
            }
            InsufficientStorage(details) => {
                e.set("code", "507".to_string());
                e.set("details", details.as_str());
            }
            LoopDetected(details) => {
                e.set("code", "508".to_string());
                e.set("details", details.as_str());
            }
            NotExtended(details) => {
                e.set("code", "510".to_string());
                e.set("details", details.as_str());
            }
            NetworkAuthenticationRequired(details) => {
                e.set("code", "511".to_string());
                e.set("details", details.as_str());
            }
            NetworkConnectTimeoutError(details) => {
                e.set("code", "599".to_string());
                e.set("details", details.as_str());
            }
        })
    }
}
