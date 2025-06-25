use clap::{Arg, Command};
use std::path::PathBuf;
use yinc::{Config, IncludeProcessor, Result};

const VERSION: &str = "0.3.1";

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("yinc")
        .version(VERSION)
        .about("YAML include processor - Rust implementation")
        .arg(
            Arg::new("file")
                .help("Input YAML file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("indent-width")
                .short('w')
                .long("indent-width")
                .value_name("WIDTH")
                .help("Indent width")
                .default_value("2"),
        )
        .arg(
            Arg::new("multi-documents")
                .short('m')
                .long("multi-documents")
                .help("Output multiple documents")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("include-tag")
                .long("include-tag")
                .value_name("TAG")
                .help("Specify include tag")
                .default_value("!include"),
        )
        .arg(
            Arg::new("replace-tag")
                .long("replace-tag")
                .value_name("TAG")
                .help("Specify replace tag")
                .default_value("!replace"),
        )
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();
    let input_path = PathBuf::from(file_path);

    let config = Config {
        indent_width: matches
            .get_one::<String>("indent-width")
            .unwrap()
            .parse()
            .unwrap_or(2),
        output_multi_documents: matches.get_flag("multi-documents"),
        include_tag: matches.get_one::<String>("include-tag").unwrap().to_string(),
        replace_tag: matches.get_one::<String>("replace-tag").unwrap().to_string(),
    };

    let processor = IncludeProcessor::new(config);
    let result = processor.process_file(&input_path).await?;
    
    println!("{}", result);
    
    Ok(())
}
