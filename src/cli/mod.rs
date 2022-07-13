use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "Hulk")]
#[clap(about = "Generate, transpile JSON files to Bicep-friendly JSON", long_about = None)]
pub struct Cli {
    /// Path to the directory or the file to count the lines from.
    pub src_path: String,

    /// The exension of the files to count. (excluding all other file types)
    #[clap(short, long, value_parser)]
    pub extension: Option<String>,
}
