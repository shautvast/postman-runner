use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use postman_runner::collection::{Collection, Event, Item, Request, Script};
use postman_runner::environment::Environment;

#[tokio::test]
async fn test() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let mock_server = MockServer::builder().listener(listener).start().await;

    Mock::given(method("GET"))
        .and(path("/get"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let collection = Collection::new(
        Item::new("test",
                  Request::new("http://localhost:8080/get"),
                  Event::new(Script::new(r#"console.log("hello world")"#))));
    // println!("{:?}",collection);
    postman_runner::run(collection, Environment::empty()).expect("")
}