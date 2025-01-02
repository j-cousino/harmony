use harmony::index::Index;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index = Index::new(".");
    let encoded: Vec<u8> = bincode::serialize(&index)?;
    let decoded: Index = bincode::deserialize(&encoded[..])?; 
    decoded.iter()
        .for_each(|(path,hash)| println!("{} ({})", path.display(), hash));
    Ok(())
}

