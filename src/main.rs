use deno_core::{JsRuntime, RuntimeOptions};
use postman_runner::{collection, environment};

// fn main() {
//     // let mut js = JsRuntime::new(RuntimeOptions::default());
//     // js.execute_script("","console.log('hello world!')").expect("");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection = collection::read_from_file("tests/json/call_http.json").expect("cannot read json");
    let env = environment::read_from_file("tests/json/simple_env.json").expect("");
    collection.run(env).await?;

    Ok(())
}

