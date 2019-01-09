use serlo_he_spec::Plugins;
use serlo_he_spec_meta::identifier_from_locator;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod files;
mod generate;

use crate::generate::{typed_attribute_list, plugin_package_imports};

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

        /// List required files which will not be generated.
        #[structopt(short = "r", long = "requirements")]
        requirements: bool,

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
            ref directory,
            requirements,
            write,
        } => {
            let plugin = spec
                .plugins
                .iter()
                .find(|p| p.identifier.name.ends_with(plugin_name))
                .expect(&format!("no plugin with name {:?}!", plugin_name));
            let identifier = identifier_from_locator(&plugin.identifier.name);

            let mut files = vec![
                files::GeneratedFile {
                    path: PathBuf::from("./package_json.patch"),
                    content: format!(
                        PACKAGE_JSON_PATCH!(),
                        &plugin.identifier.name,
                        &plugin.identifier.version.to_string()
                    ),
                },
                files::GeneratedFile {
                    path: PathBuf::from("src/index.ts"),
                    content: format!(
                        REACT_DEFINITION!(),
                        &format!("{}Edit", &identifier),
                        "editor",
                        &format!("{}Edit", &identifier),
                        &plugin.description
                    ),
                },
                files::GeneratedFile {
                    path: PathBuf::from("src/index.renderer.ts"),
                    content: format!(
                        REACT_DEFINITION!(),
                        &format!("{}Renderer", &identifier),
                        "renderer",
                        &format!("{}Renderer", &identifier),
                        &plugin.description
                    ),
                },
                files::GeneratedFile {
                    path: PathBuf::from("src/state.ts"),
                    content: {
                        format!(
                            STATE!(),
                            &plugin_package_imports(&plugin, &spec)
                                .expect("error generating state dependencies!")
                                .join("\n"),
                            &identifier,
                            &typed_attribute_list(&plugin, &spec)
                                .expect("error generating attributes!")
                                .join(",\n")
                        )
                    },
                },
                files::GeneratedFile {
                    path: PathBuf::from("./README.md"),
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

            for mut file in &mut files {
                file.path = directory.join(&file.path);
            }

            if requirements {
                let requirements = vec![
                    "package.json",
                    "LICENSE",
                    "src/plugin.ts",
                    "babel.config.js",
                    "tsconfig.json",
                ];
                for req in requirements {
                    println!("{}", req);
                }
                return;
            }

            if write {
                for file in files {
                    let base_dir = PathBuf::from(&file.path.parent().unwrap());
                    fs::create_dir_all(&base_dir).expect("could not create output directory!");
                    let mut f = fs::File::create(&file.path)
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
