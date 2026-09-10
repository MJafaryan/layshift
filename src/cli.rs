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
        about = "Maps text between two keyboard layouts",
        long_about = "Maps text between two keyboard layouts\n\
                      If you have set default layouts, just run:\n\
                        `layshift map`\n\
                      And for mapping between two layouts in general:\n\
                        `layshift map <source_layout> <target_layout>`\n\
                      Example:\n\
                        `layshift map fa:winkey en:qwerty`"
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
                        `layshift set-default <source_layout> <target_layout>`\n\
                      Example:\n\
                        `layshift set-default en:qwerty fa:winkey`"
    )]
    SetDefault {
        /// Source keyboard layout
        source: String,
        /// Target keyboard layout
        target: String,
    },
    #[command(
        about = "List available languages and layouts",
        long_about = "Show list of languages/layouts\n\
                      For showing languages list run:\n\
                        `layshift list`\n\
                      And for showing a language layouts list run:\n\
                        `layshift list <language>`\n\
                      Example:\n\
                        `layshift list english`\n\
                      Or just use the language symbol:\n\
                        `layshift list en`"
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
            _ => return Err("Both source and target layouts are required.".into()),
        };

        let source_layout = layout::Layout::new(&source)?;
        let target_layout = layout::Layout::new(&target)?;

        let text = clipboard::read()?;
        let result = layout::map_string(&text, &source_layout, &target_layout);

        clipboard::write(&result)?;
        Ok(())
    }

    fn set_default(source: String, target: String) -> Result<(), Box<dyn std::error::Error>> {
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
                    print!(
                        "{:<30} {} layouts\n",
                        format!("{}({})", language.name, language.symbol),
                        language.layouts.len()
                    );
                }
            }
            Some(language) => {
                let layouts = metadata::get_language_layouts(&language)?;

                for layout in layouts {
                    print!("{} ", layout);
                }
                print!("\n")
            }
        }
        Ok(())
    }
}
