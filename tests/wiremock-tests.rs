use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use std::error::Error;
use postman_runner::{collection, environment};

#[tokio::test]
async fn test_get() -> Result<(), Box<dyn Error + Send + Sync>>{
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let mock_server = MockServer::builder().listener(listener).start().await;
    
    Mock::given(method("GET"))
        .and(path("/headers"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let collection = collection::read_from_file("tests/json/call_http.json").expect("cannot read json");
    let env = environment::read_from_file("tests/json/env-local.json").expect("");
    collection.run(env)?;

    
    Ok(())
}
