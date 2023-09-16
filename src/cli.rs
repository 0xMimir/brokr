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
