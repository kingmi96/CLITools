use clap::{Parser, Subcommand};
mod url_shortener;
mod password_generator;
mod image_resizer;
mod text_analyzer;
mod system_info;
use url_shortener::run_url_shortener;
use password_generator::{run_password_generator, PasswordOptions};
use image_resizer::{run_image_resizer, ResizeOptions};
use text_analyzer::{run_text_analyzer, AnalyzerOptions};
use system_info::{run_system_info, InfoOptions};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    UrlShortener,
    PasswordGenerator(PasswordOptions),
    ImageResizer(ResizeOptions),
	TextAnalyzer(AnalyzerOptions),
	SystemInfo(InfoOptions),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::UrlShortener) => run_url_shortener(),
        Some(Commands::PasswordGenerator(options)) => run_password_generator(options),
        Some(Commands::ImageResizer(options)) => run_image_resizer(options),
		Some(Commands::TextAnalyzer(options)) => run_text_analyzer(options),
		Some(Commands::SystemInfo(options)) => run_system_info(options),
		None => println!("No command provided. Use --help to see available commands."),
    }
}

