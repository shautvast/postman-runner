let environment = {
    name: 'Test',
    has: key => this[key] !== undefined
};

let tests = {};
const run_tests = () => {
    for (const test in tests) {
        let result;
        if (tests[test] === false || tests[test] === true) {
            //old api
            result = tests[test];
        } else {
            // new api
            result = tests[test]();
        }
        if (result !== true) {
            console.error("test '" + test + "' failed");
            return "failure";
        }
    }

    return "succes";
};
let pm = {
    environment: environment,

    test: (name, fn) => {
        tests[name] = fn;
    },

    setEnvironmentVariable: (key, value) => {
        pm[key] = value;
    }
};