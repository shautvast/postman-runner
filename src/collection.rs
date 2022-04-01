use crate::environment::Environment;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use failure::format_err;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Collection {
    pub variables: Vec<String>,
    pub info: Info,
    pub item: Vec<Item>,
}

impl Collection {
    pub async fn run(
        &self,
        env: Environment,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        for it in self.item.iter() {
            for it in it.item.iter() {
                for req in it.request.iter() {
                    let mut request_builder = client.get(env.resolve(&req.url).expect("url not valid"));
                    for h in req.header.iter() {
                        request_builder = request_builder.header(
                            &env.resolve(&h.key).expect("header key not valid"),
                            &env.resolve(&h.value).expect("header value not valid"),
                        );
                    }

                    if let Some(raw) = &req.body.as_ref().unwrap().raw {
                        let resolved_body = env
                            .resolve(raw)
                            .expect("cannot resolve body");
                        request_builder = request_builder
                            .body(resolved_body);
                           
                    }
                    let response = request_builder.send().await?;
                    if response.status() !=200{
                        return Err(format_err!("Result {}", response.status()).into());
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn read_from_file(file_name: &str) -> Result<Collection, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let collection = serde_json::de::from_reader(reader)?;
    Ok(collection)
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Info {
    pub name: String,
    pub description: String,
    pub schema: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "empty_item_list")]
    pub item: Vec<Item>,
    #[serde(default = "empty_event_list")]
    pub event: Vec<Event>,
    pub request: Option<Request>,
}

fn empty_item_list() -> Vec<Item> {
    Vec::new()
}
fn empty_event_list() -> Vec<Event> {
    Vec::new()
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Event {
    listen: String,
    script: Script,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Script {
    r#type: String,
    exec: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Request {
    url: String,
    method: Method,
    header: Vec<Header>,
    body: Option<Body>,
    description: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Header {
    key: String,
    value: String,
    description: String,
}

#[derive(Deserialize, Debug, PartialEq)]
enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Body {
    mode: Option<String>,
    raw: Option<String>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_collection() {
        let collection = read_from_file("tests/json/json_read_test.json").expect("");
        assert_eq!(collection.info.name, "pCurrentAccountsOpenAPI ICHP");
    }
}
