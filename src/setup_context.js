let environment = {
    name: 'Test',
    has: key =>  this[key] !== undefined
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
    setEnvironmentVariable: function (key, value) {
        pm[key] = value;
    }
};