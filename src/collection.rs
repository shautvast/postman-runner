use crate::environment::Environment;
use crate::javascript;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use failure::format_err;
use quick_js::Context;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Collection {
    pub variables: Vec<String>,
    pub info: Option<Info>,
    pub item: Vec<Item>,
}

impl Collection {
    pub fn new(item: Item) -> Self {
        Self { variables: vec![], info: None, item: vec![item] }
    }

    pub fn run(
        &self,
        env: Environment,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let js = javascript::new_runtime()?;

        for item in self.item.iter() {
            handle_item(item, &env, &js)?;
        }
        Ok(())
    }
}

fn handle_item(parent_item: &Item, env: &Environment, js: &Context) -> Result<(), Box<dyn std::error::Error+ Send + Sync>> {
    for item in parent_item.item.iter() {
        handle_item(item, env, js)?;
    }

    // requests
    for req in parent_item.request.iter() {
        let mut builder = ureq::get(&env.resolve(&req.url).expect("url not valid"));

        for h in req.header.iter() {
            builder = builder.set(
                &env.resolve(&h.key).expect("header key not valid"),
                &env.resolve(&h.value).expect("header value not valid"),
            );
        }

        let response = if let Some(body) = &req.body {
            if let Some(raw) = body.raw.as_ref() {
                let resolved_body = env
                    .resolve(raw)
                    .expect("cannot resolve body");
                builder.send_string(&resolved_body).expect("cannot execute http request")
            } else {
                builder.call().expect("cannot execute http request")
            }
        } else {
            builder.call().expect("cannot execute http request")
        };

        js.eval(&format!("let responseCode={{code:{}}}", response.status()))?;

        // events
        for event in parent_item.event.iter() {
            for script in event.script.exec.iter() {
                js.eval(script)?;
            }

            if event.listen == "test"{
                let result = js.eval("run_tests()")?;
                if result.as_str().unwrap() == "failure"{
                    return Err(format_err!("test failure").into());
                }
            }
        }

        // if response.status() != 200 {
        //     return Err(format_err!("Result {}", response.status()).into());
        // }
    }

    Ok(())
}


pub fn read_from_file(file_name: &str) -> Result<Collection, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let collection = serde_json::de::from_reader(reader)?;
    Ok(collection)
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Info {
    pub name: String,
    pub description: String,
    pub schema: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "empty_item_list")]
    pub item: Vec<Item>,
    #[serde(default = "empty_event_list")]
    pub event: Vec<Event>,
    pub request: Option<Request>,
}

impl Item {
    pub fn new(name: &str, request: Request, event: Event) -> Self {
        Self {
            name: name.to_owned(),
            description: None,
            item: vec![],
            event: vec![event],
            request: Some(request),
        }
    }
}

fn empty_item_list() -> Vec<Item> {
    Vec::new()
}

fn empty_event_list() -> Vec<Event> {
    Vec::new()
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Event {
    listen: String,
    script: Script,
}

impl Event {
    pub fn new(script: Script) -> Self {
        Self {
            listen: "test".to_owned(),
            script,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Script {
    r#type: String,
    exec: Vec<String>,
}

impl Script {
    pub fn new(code: &str) -> Self {
        Self {
            r#type: "application/javascript".to_owned(),
            exec: vec![code.to_owned()],
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Request {
    url: String,
    method: Method,
    header: Vec<Header>,
    body: Option<Body>,
    description: Option<String>,
}

impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
            method: Method::GET,
            header: vec![],
            body: None,
            description: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Header {
    key: String,
    value: String,
    description: Option<String>,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            key: name.to_owned(),
            value: value.to_owned(),
            description: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Body {
    mode: Option<String>,
    //other modes than 'raw'?
    raw: Option<String>,
}

impl Body {
    pub fn new(body: &str) -> Self {
        Self {
            mode: Some("raw".to_owned()), //option?
            raw: Some(body.to_owned()), //option?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_collection() {
        let collection = read_from_file("tests/json/json_read_test.json").expect("");
        assert_eq!(collection.info.unwrap().name, "pCurrentAccountsOpenAPI ICHP");
    }
}
