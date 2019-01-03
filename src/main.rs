use serlo_he_spec::{Heading, Plugins};
use serlo_he_spec_meta::identifier_from_locator;
use structopt::StructOpt;
use std::path::PathBuf;

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
    },
}

fn main() {
    let args = Args::from_args();
    match args {
        Args::Generate { ref plugin_name, ref write } => {
            let spec = Plugins::whole_specification();
            let plugin = spec
                .plugins
                .iter()
                .find(|p| p.identifier.name.ends_with(plugin_name))
                .expect(&format!("no plugin with name {:?}!", plugin_name));
            let identifier = identifier_from_locator(&plugin.identifier.name);
            let editor_types = spec.editor_types
                .get(plugin.identifier.name.split('/').last().unwrap_or_default())
                .expect("editor types not defined for plugin!");

            let files = vec![
                files::GeneratedFile {
                    path: PathBuf::from("plugin.ts"),
                    content: format!(
                                 PACKAGE_JSON_PATCH!(),
                                 &plugin.identifier.name,
                                 &plugin.identifier.version.to_string())
                },
                files::GeneratedFile {
                    path: PathBuf::from("index.ts"),
                    content: format!(
                                 REACT_DEFINITION!(),
                                 &editor_types.editor,
                                 "editor",
                                 &editor_types.editor,
                                 &plugin.description)
                },
                files::GeneratedFile {
                    path: PathBuf::from("index.renderer.ts"),
                    content: format!(
                                 REACT_DEFINITION!(),
                                 &editor_types.renderer,
                                 "editor",
                                 &editor_types.renderer,
                                 &plugin.description)
                }
            ];
            if !write {
                for file in files {
                    println!("{:?}:", &file.path);
                    println!("{}", &file.content);
                }
            }
        }
    }
}
