pub const RENDERER_INDEX: &str = r#"
{{~ #each imports ~}}
{{this}}
{{~ /each}}
import { {{component_ident}}Renderer } from './renderer'
import { RendererPlugin } from '@splish-me/editor'

export const {{plugin_suffix}}RendererPlugin: RendererPlugin<{{component_ident}}PluginState> = {
  Component: {{component_ident}}Renderer
}

export interface {{component_ident}}PluginState {
  {{~ #each attributes}}
  {{this}}
  {{~ /each}}
}

export * from './renderer'
"#;

pub const RENDERER_PACKAGE: &str = r#"{
  "name": "{{name}}{{name_suffix}}",
  "version": "{{version}}",
  "dependencies": {
{{dependencies}}
  }
}"#;

pub const EDIT_INDEX: &str = r#"import {
  {{component_ident}}PluginState,
  {{component_ident}}Renderer
} from '{{plugin_path}}-renderer'

import { {{component_ident}}Editor } from './editor'
import { Plugin, createDocumentIdentifier } from '@splish-me/editor'

export const {{plugin_suffix}}Plugin: Plugin<{{component_ident}}PluginState> = {
  Component: {{component_ident}}Editor,
  text: '{{component_description}}',

  createInitialState: (): {{component_ident}}PluginState => (
    {{~ component_default ~}}
  )
}
"#;
