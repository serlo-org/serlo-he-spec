pub const RENDERER_INDEX: &str = r#"
{{~ #each imports}}
{{this}}
{{~ /each}}
import { {{component_ident}}Renderer } from './renderer'

export const {{plugin_suffix}}RendererPlugin = {
  name: '{{plugin_ident.name}}',
  version: '{{plugin_ident.version}}',
  Component: {{component_ident}}Renderer
}

export interface {{component_ident}}PluginState {
  {{~ #each attributes}}
  {{this}},
  {{~ /each}}
}

export * from './renderer'
"#;

pub const RENDERER_PACKAGE: &str = r#"{
  "name": "{{name}}{{name_suffix}}",
  "version": "{{version}}",
  "peerDependencies": {
{{dependencies}}
  }
}"#;

pub const EDIT_INDEX: &str = r#"
import {
  {{component_ident}}PluginState,
  {{component_ident}}Renderer
} from '{{plugin_path}}-renderer'

import { {{component_ident}}Editor } from './editor'

export const foobarPlugin = {
  name: '{{plugin_path}}',
  version: '{{plugin_version}}',
  Component: {{component_ident}}Editor,
  text: '{{component_description}}',

  createInitialState: (): {{component_ident}}PluginState => {
    return {{component_default}}
  }
}
"#;
