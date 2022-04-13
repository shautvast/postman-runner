use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use postman_runner::{collection, environment};

#[tokio::test]
async fn test_get() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let mock_server = MockServer::builder().listener(listener).start().await;
    
    Mock::given(method("GET"))
        .and(path("/get200"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let collection = collection::read_from_file("tests/json/call_http.json").expect("cannot read json");
    let env = environment::read_from_file("tests/json/env-local.json").expect("");
    check_error(collection.run(env));
    

}

fn check_error(result: Result<(), Box<dyn std::error::Error + Send + Sync>>) {
    match result {
        Ok(()) => {},
        Err(e) => {
            println!("{}", e);
            panic!("Error during unittest");
        }
    }
}
