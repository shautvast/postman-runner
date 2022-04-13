use postman_runner::{collection, environment};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Wrong number of arguments!\n\nUsage: \npmrunner COLLECTION_FILENAME ENV_FILENAME\nBoth files must be standard postman 2.1 json files\n");
    }

    let collection_file = &args[1];
    let environment_file = &args[2];

    read_files_and_run(collection_file,environment_file)
}

fn read_files_and_run(collection_file: &str, env_file: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let collection = collection::read_from_file(collection_file).expect("cannot read collection json");
    let env = environment::read_from_file(env_file).expect("cannot read environment json");

    postman_runner::run(collection, env)
}

