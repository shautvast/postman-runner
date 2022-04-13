pub mod collection;
pub mod environment;
mod javascript;

pub fn run(collection: collection::Collection, env: environment::Environment) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    collection.run(env)
}


