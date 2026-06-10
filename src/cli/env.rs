use anyhow::Result;
use std::env;

pub fn list_vars() {
    println!("Environment variables:");
    for (key, value) in env::vars() {
        println!("  {} = {}", key, value);
    }
}

pub fn get_var(name: &str) -> Result<()> {
    match env::var(name) {
        Ok(value) => println!("{} = {}", name, value),
        Err(_) => println!("Variable '{}' not found", name),
    }
    Ok(())
}

pub fn set_var(name: &str, value: &str) -> Result<()> {
    env::set_var(name, value);
    println!("Set {} = {}", name, value);
    Ok(())
}
