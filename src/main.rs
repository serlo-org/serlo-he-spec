use serlo_he_spec::Plugins;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod files;

#[derive(StructOpt)]
enum Args {
    /// generate plugin source code for the serlo editor.
    #[structopt(name = "generate")]
    Generate {
        /// name of the plugin.
        #[structopt(name = "plugin")]
        plugin_name: String,

        /// Write the generated plugin files to disk.
        #[structopt(short = "w", long = "write")]
        write: bool,

        /// Root directory to write files to.
        #[structopt(
            short = "d",
            long = "directory",
            parse(from_os_str),
            default_value = ""
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
            ref write,
            ref directory,
        } => {
            let plugin = spec
                .plugins
                .iter()
                .find(|p| p.identifier.name.ends_with(plugin_name))
                .expect(&format!("no plugin with name {:?}!", plugin_name));
            let editor_types = spec
                .editor_types
                .get(plugin.identifier.name.split('/').last().unwrap_or_default())
                .expect("editor types not defined for plugin!");

            let files = vec![
                files::GeneratedFile {
                    path: PathBuf::from("package_json.patch"),
                    content: format!(
                        PACKAGE_JSON_PATCH!(),
                        &plugin.identifier.name,
                        &plugin.identifier.version.to_string()
                    ),
                },
                files::GeneratedFile {
                    path: PathBuf::from("index.ts"),
                    content: format!(
                        REACT_DEFINITION!(),
                        &editor_types.editor, "editor", &editor_types.editor, &plugin.description
                    ),
                },
                files::GeneratedFile {
                    path: PathBuf::from("index.renderer.ts"),
                    content: format!(
                        REACT_DEFINITION!(),
                        &editor_types.renderer,
                        "editor",
                        &editor_types.renderer,
                        &plugin.description
                    ),
                },
                files::GeneratedFile {
                    path: PathBuf::from("README.md"),
                    content: format!(
                        README!(),
                        &plugin.identifier.name,
                        &plugin
                            .attributes
                            .iter()
                            .map(|a| a.identifier.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                        &plugin.documentation
                    ),
                },
            ];
            if *write {
                for file in files {
                    fs::create_dir_all(&directory).expect("could not create output directory!");
                    let mut f = fs::File::create(&directory.join(&file.path))
                        .expect(&format!("could not create {:?}", &file.path));
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
