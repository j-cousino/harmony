use harmony::index::Index;

fn main() {
    let index = Index::new(".");
    index.iter()
        .for_each(|(path,hash)| println!("{}\n{}", path.display(), hash));
}
