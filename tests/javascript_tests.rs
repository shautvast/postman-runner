use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use postman_runner::collection::{Collection, Event, Item, Request, Script};
use postman_runner::environment::Environment;

#[tokio::test]
async fn all_tests() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let mock_server = MockServer::builder().listener(listener).start().await;

    Mock::given(method("GET"))
        .and(path("/get200"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/get201"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    expect_and_get200();
    expect_200_and_get201();
}

fn expect_and_get200() {
    let collection = Collection::new(
        Item::new("test",
                  Request::new("http://localhost:8080/get200"),
                  Event::new(Script::new(r#"tests["expect200"] = responseCode.code === 200"#))));
    postman_runner::run(collection, Environment::empty()).expect("")
}


fn expect_200_and_get201() {
    cool_asserts::assert_panics!({
    let collection = Collection::new(
        Item::new("test",
                  Request::new("http://localhost:8080/get201"),
                  Event::new(Script::new(r#"tests["expect200"] = responseCode.code === 200"#))));
    postman_runner::run(collection, Environment::empty()).expect("")
    });
}