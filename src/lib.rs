use quick_js::{Context, JsValue};

pub mod collection;
pub mod environment;

pub fn run(collection: collection::Collection, env: environment::Environment) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    collection.run(env)


    // let result = js.eval(r#"function a(){
    // console.log("hello world");
    // let responseBody='{"ptoPToken":"token"}';
    // pm.setEnvironmentVariable("ptoPToken", JSON.parse(responseBody).ptoPToken);
    //
    // return JSON.stringify(pm);
    // } a();"#).expect("");
    // let value = result;
    // println!("{:?}", value);
    // Ok(())
}


