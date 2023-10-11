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

    let max_threads = cli.get_one::<u8>("THREADS").copied();

    let brokr = Brokr::new();
    let links = brokr.find_links(source_dir, &file_extensions)?;
    let invalid_links = brokr.find_broken_links(links, max_threads);

    if !invalid_links.is_empty() {
        println!("\nFound {} invalid links\n", invalid_links.len());
        for invalid_link in invalid_links {
            println!("{}", invalid_link);
        }
    }

    Ok(())
}
