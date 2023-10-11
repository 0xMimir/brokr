use brokr::Brokr;
mod cli;

#[cfg(not(tarpaulin_include))]
fn main() -> anyhow::Result<()> {
    let cli = cli::build_cli().get_matches();
    let source_dir = cli
        .get_one::<String>("SOURCE_DIR")
        .expect("Default value not set"); // This is safe due to default value

    let file_extensions = cli
        .get_many::<String>("EXTENSIONS")
        .expect("Default value not set")
        .map(|e| e.as_str())
        .collect::<Vec<_>>();

    let brokr = Brokr::new();
    let _links = brokr.find_links(source_dir, &file_extensions)?;

    // if !_invalid_links.is_empty() {
    //     println!("\nFound {} invalid links\n", _invalid_links.len());
    //     for invalid_link in _invalid_links.iter() {
    //         println!("{}", invalid_link);
    //     }
    // }

    Ok(())
}
