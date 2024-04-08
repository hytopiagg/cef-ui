use std::process::exit;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);

        exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    Ok(())
}
