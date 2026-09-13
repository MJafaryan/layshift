use std::fs;

use clap::{Parser, Subcommand};

use crate::{clipboard, config, layout, metadata};

#[derive(Parser, Debug)]
#[command(name = "layshift")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        about = "Map text between two keyboard layouts",
        long_about = "Map text between two keyboard layouts\n\
                      To map between two layouts:\n\
                      \tlayshift map <source_layout> <target_layout>\n\n\
                      Example:\n\
                      \tlayshift map fa:winkey en:qwerty\n\n\
                      If you have set default layouts, just run:\n\
                      \tlayshift map\n"
    )]
    Map {
        /// Source keyboard layout
        source: Option<String>,
        /// Target keyboard layout
        target: Option<String>,
    },
    #[command(
        about = "Set two layouts as default layouts",
        long_about = "Set two layouts as default layouts\n\
                      \tlayshift set-default <source_layout> <target_layout>\n\n\
                      Example:\n\
                      \tlayshift set-default en:qwerty fa:winkey"
    )]
    SetDefault {
        /// Source keyboard layout
        source: String,
        /// Target keyboard layout
        target: String,
    },
    #[command(
        about = "List available languages and layouts",
        long_about = "List available languages and layouts\n\
                      To list all available languages, run:\n\
                      \tlayshift list\n\n\
                      To list layouts for a language:\n\
                      \tlayshift list <language>\n\n\
                      Example:\n\
                      \tlayshift list english\n\n\
                      Or use the language symbol:\n\
                      \tlayshift list en"
    )]
    List {
        /// Language
        language: Option<String>,
    },
}

impl Cli {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Map { source, target } => Cli::map(source, target),
            Commands::SetDefault { source, target } => Cli::set_default(source, target),
            Commands::List { language } => Cli::list(language),
        }
    }

    fn map(
        source: Option<String>,
        target: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (source, target) = match (source, target) {
            (None, None) => config::get_default_layouts()?,
            (Some(source), Some(target)) => (source, target),
            _ => return Err("Provide both source and target layouts, or neither.".into()),
        };

        let source_layout = layout::Layout::new(&source)?;
        let target_layout = layout::Layout::new(&target)?;

        let text = clipboard::read()?;
        let result = layout::map_string(&text, &source_layout, &target_layout);

        clipboard::write(&result)?;
        Ok(())
    }

    fn set_default(source: String, target: String) -> Result<(), Box<dyn std::error::Error>> {
        // Validating input layouts
        layout::Layout::new(&source)?;
        layout::Layout::new(&target)?;

        let result = format!("source = \"{}\"\ntarget = \"{}\"\n", source, target);

        fs::create_dir_all(config::get_config_dir())?;
        fs::write(config::get_config_file(), result)?;

        Ok(())
    }

    fn list(language: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        match language {
            None => {
                let languages = metadata::get_languages_list()?;

                for language in languages {
                    println!(
                        "{:<30} {} layouts",
                        format!("{}({})", language.name, language.symbol),
                        language.layouts.len()
                    );
                }
            }
            Some(language) => {
                let layouts = metadata::get_language_layouts(&language)?;

                for layout in layouts {
                    println!("{layout}");
                }
            }
        }
        Ok(())
    }
}
