#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cargo = tokio::fs::read_to_string("./Cargo.toml").await?;
  
    println!("Cargo.toml content:\n{cargo}");
  
    Ok(())
} 