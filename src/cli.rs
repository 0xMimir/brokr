use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "brokr")]
pub struct CargoBrokrCli {
    /// Specifies the folder in which to find broken urls
    #[arg(long, default_value = ".")]
    pub source_dir: String,

    /// File extensions to search for
    #[arg(long, default_values_t = vec!["html".to_owned(), "md".to_owned(), "txt".to_owned()])]
    pub extensions: Vec<String>,

    /// Number of threads to spawn to check broken urls
    #[arg(long)]
    pub threads: Option<u8>,

    /// Test links for localhost
    #[arg(long, default_value_t = false)]
    pub test_localhost: bool,
}
