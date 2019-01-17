use serlo_he_spec::Plugins;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod editor_ts;
mod files;

#[derive(StructOpt)]
/// Generate various data from the serlo higher education plugin specification.
enum Args {
    /// generate plugin source code for the serlo editor.
    #[structopt(name = "generate")]
    Generate {
        /// Name of the plugin.
        #[structopt(name = "plugin")]
        plugin_name: String,

        /// Print generated files instead of writing to disk.
        #[structopt(short = "n", long = "--dry-run")]
        dry: bool,

        /// Directory to write plugins to.
        #[structopt(
            short = "d",
            long = "directory",
            parse(from_os_str),
            default_value = "."
        )]
        directory: PathBuf,
    },
    /// List all plugins.
    #[structopt(name = "list")]
    List {},
}

fn main() {
    let args = Args::from_args();
    let spec = Plugins::whole_specification();
    match args {
        Args::Generate {
            ref plugin_name,
            ref directory,
            dry,
        } => {
            let plugin = spec
                .plugins
                .iter()
                .find(|p| p.identifier.name.ends_with(plugin_name))
                .unwrap_or_else(|| panic!("no plugin with name {:?}!", plugin_name));
            let mut files = editor_ts::editor_plugin_files(plugin).expect("generation error:");

            for mut file in &mut files {
                file.path = directory.join(&file.path);
            }

            if !dry {
                for file in files {
                    let base_dir = PathBuf::from(&file.path.parent().unwrap());
                    fs::create_dir_all(&base_dir).expect("could not create output directory!");
                    let mut f = fs::File::create(&file.path)
                        .unwrap_or_else(|_| panic!("could not create {:?}", &file.path));
                    write!(&mut f, "{}", &file.content).expect("could not write to output!");
                }
            } else {
                for file in files {
                    println!("{:?}:", &file.path);
                    println!("{}", &file.content);
                }
            }
        }
        Args::List {} => {
            for plugin in &spec.plugins {
                println!("{}", &plugin.identifier.name);
            }
        }
    }
}
