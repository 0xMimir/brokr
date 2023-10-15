use brokr::Brokr;
mod cli;
use crate::cli::CargoBrokrCli;
use clap::Parser;

#[cfg(not(tarpaulin_include))]
fn main() -> anyhow::Result<()> {
    let config = match CargoBrokrCli::try_parse() {
        Ok(config) => config,
        Err(e) => e.exit(),
    };

    let brokr = Brokr::new();
    let links = brokr.find_links(
        &config.source_dir,
        &config.extensions,
        config.test_localhost,
    )?;

    println!("Found {} links", links.len());

    let invalid_links = brokr.find_broken_links(links, config.threads, vec![]);

    if !invalid_links.is_empty() {
        println!("\nFound {} invalid links\n", invalid_links.len());
        for invalid_link in invalid_links {
            println!("{}", invalid_link);
        }
    }

    Ok(())
}
