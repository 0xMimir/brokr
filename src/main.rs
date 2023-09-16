use brokr::Brokr;
mod cli;

fn main() {
    let cli = cli::build_cli().get_matches();
    let source_dir = cli.get_one::<String>("SOURCE_DIR").unwrap(); // This is safe due to default value
    let file_extensions = cli
        .get_many::<String>("EXTENSIONS")
        .unwrap()
        .collect::<Vec<_>>();

    let brokr = Brokr::new();
    brokr.run(source_dir, &file_extensions).unwrap();
}
