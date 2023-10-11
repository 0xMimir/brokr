use clap::{Arg, Command};

const DEFAULT_EXTENSIONS: [&str; 3] = ["html", "md", "txt"];

pub fn build_cli() -> Command {
    let src_dir = Arg::new("SOURCE_DIR")
        .long("source-dir")
        .default_value(".")
        .help("Specifies the folder in which to find broken urls");

    let file_extensions = Arg::new("EXTENSIONS")
        .long("extensions")
        .default_values(DEFAULT_EXTENSIONS)
        .value_delimiter(',')
        .help("File extensions to search for");

    Command::new("brokr").arg(src_dir).arg(file_extensions)
}

#[test]
fn test_set_default_values() {
    let command = build_cli().get_matches();

    let source_dir = command
        .get_one::<String>("SOURCE_DIR")
        .expect("Default value not set")
        .as_str();

    assert_eq!(source_dir, ".");

    let file_extensions = command
        .get_many::<String>("EXTENSIONS")
        .expect("Default value not set")
        .map(|e| e.as_str())
        .collect::<Vec<_>>();

    for extension in DEFAULT_EXTENSIONS {
        assert!(file_extensions.contains(&extension));
    }
}
