use std::error::Error;
use quick_js::{Context, JsValue};

pub(crate) fn new_runtime() -> Result<Context, Box<dyn Error + Send + Sync>> {
    let context = Context::builder().console(|_, args: Vec<JsValue>| {
        for jsv in args {
            print!("{:?} ", jsv);
        }
        println!();
    }).build().unwrap();
    setup_js_context(&context)?;
    Ok(context)
}

fn setup_js_context(js: &Context) -> Result<(), Box<dyn Error + Send + Sync>> {
    let js_setup_file = include_bytes!("setup_context.js");
    js.eval(&String::from_utf8_lossy(js_setup_file).to_string())?;

    Ok(())
}