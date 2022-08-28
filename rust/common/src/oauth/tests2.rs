#[cfg(test)]
mod tests {
    use async_std::main;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn oauth_api_tests() {
        #[::tokio::main]
        async fn main() {
            // Start a background HTTP server on a random local port
            let mock_server = MockServer::start().await;

            // Arrange the behaviour of the MockServer adding a Mock:
            // when it receives a GET request on '/hello' it will respond with a 200.
            Mock::given(method("GET"))
                .and(path("/hello"))
                .respond_with(ResponseTemplate::new(200))
                // Mounting the mock on the mock server - it's now effective!
                .mount(&mock_server)
                .await;

            // If we probe the MockServer using any HTTP client it behaves as expected.
            let status = surf::get(format!("{}/hello", &mock_server.uri()))
                .await
                .unwrap()
                .status();
            assert_eq!(status, 200);

            // If the request doesn't match any `Mock` mounted on our `MockServer` a 404 is returned.
            let status = surf::get(format!("{}/missing", &mock_server.uri()))
                .await
                .unwrap()
                .status();
            assert_eq!(status, 404);
        }
    }
}
