mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() {
    if let Err(e) = commands::Command::handle() {
        eprintln!("Error: {}", &e);
    }
}
