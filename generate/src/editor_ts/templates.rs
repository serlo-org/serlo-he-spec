pub const RENDERER_INDEX: &'static str = r#"
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

pub const RENDERER_PACKAGE: &'static str = r#"{
  "name": "{{name}}",
  "version": "{{version}}",
  "peerDependencies": {
{{dependencies}}
  }
}"#;
