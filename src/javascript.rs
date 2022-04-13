use quick_js::{Context, JsValue};

pub(crate) fn new_runtime() -> Context {
    let context = Context::builder().console(|_, args: Vec<JsValue>| {
        for jsv in args {
            print!("{} ", jsv.as_str().unwrap());
        }
        println!();
    }).build().unwrap();
    setup_js_context(&context);
    context
}

fn setup_js_context(js: &Context) {
    js.eval(r#"
    let environment = {
        name: 'Test',
        has: key => {
            this[key] !== undefined
        }
    };

    let tests = {};
    const run_tests = () => {
        for (const test in tests) {
            if (tests[test] !== true) {
                console.error("test '" + test + "' failed");
                return "failure";
            }
        }

        return "succes";
    };
    let pm = {
        environment: environment,
        setEnvironmentVariable: function(key,value){
            pm[key]=value;
        }
    };"#).expect("");
}