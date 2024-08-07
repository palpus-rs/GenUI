use std::{collections::HashMap, hash::Hash};

use gen_converter::model::{
    prop::ConvertStyle,
    script::{CurrentInstance, GenScriptModel, PropFn, ScriptModel, UseMod},
    Source, TemplateModel,
};
use gen_parser::{PropsKey, Value};

use gen_utils::common::{ident, snake_to_camel};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Ident, ItemEnum, ItemStruct, Stmt, StmtMacro};

use crate::{
    utils::{component_render, special_struct},
    widget::{
        utils::{combine_option, quote_draw_widget},
        BuiltIn,
    },
};

use super::{
    handler::WidgetHandler, live_hook::LiveHookTrait, role::Role, traits::WidgetTrait, ToLiveDesign,
};

/// ## 当生成 live_design! 中的节点时
/// `[id] [:|=] <name>{ [...props|widget...] }`
/// ## 当生成一个完整的组件时
#[derive(Debug, Default, Clone)]
pub struct Widget {
    /// Makepad live_design! macro
    // pub live_design: Option<LiveDesign>,
    pub is_root: bool,
    pub is_prop: bool,
    pub is_built_in: bool,
    /// is a define widget
    pub is_static: bool,
    /// widget id, if widget is prop, id is prop
    pub id: Option<String>,
    /// is widget as a prop? if prop is true , widget need id
    /// `<view id="a" as_prop></view>` => as_prop = true
    pub as_prop: bool, 
    pub name: String,
    pub source: Option<Source>,
    pub imports: Option<TokenStream>,
    pub uses: Option<TokenStream>,
    // pub compiled_source: Option<PathBuf>,
    /// props in live_design
    pub props: Option<TokenStream>,
    /// events called in makepad
    pub events: Option<HashMap<String, TokenStream>>,
    pub prop_ptr: Option<TokenStream>,
    pub event_ptr: Option<TokenStream>,
    pub event_ref: Option<TokenStream>,
    pub event_set: Option<TokenStream>,
    pub children: Option<Vec<Widget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: Option<WidgetTrait>,
    pub live_hook: Option<LiveHookTrait>,
    pub role: Role,
}

impl PartialEq for Widget {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for Widget {}

impl Hash for Widget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
    }
}

impl Widget {
    pub fn default_ui_root() -> Self {
        let mut widget = Widget::default();

        widget.set_is_root(true).set_id(Some(&"ui".to_string()));
        widget.name = "Root".to_string();
        widget.is_static = true;

        widget
    }
    pub fn new(special: Option<Source>, name: &str, inherits: Option<&String>) -> Self {
        let mut widget = Widget::default();
        match special {
            Some(special) => {
                widget.source.replace(special);
                let inherits_widget = BuiltIn::try_from(inherits).unwrap();
                // 获取文件名且改为首字母大写的camel
                match inherits {
                    Some(_) => {
                        widget.name = widget.source.as_ref().unwrap().source_name();
                    }
                    None => {
                        // 首个节点没有inherits且name不是`component`
                        if name.eq("component") {
                            widget.name = inherits_widget.to_string();
                        } else {
                            widget.name = BuiltIn::try_from(name).unwrap().to_string();
                            widget.set_is_built_in(true);
                        }
                    }
                }
                widget.set_inherits(inherits_widget);
            }
            None => {
                widget.name = name.to_string();
                if let Ok(inherits) = BuiltIn::try_from(name) {
                    widget.set_is_built_in(true).set_inherits(inherits);
                }
            }
        }
        widget.set_traits(WidgetTrait::default());
        widget.live_hook.replace(LiveHookTrait::default());
        widget
    }
    pub fn new_builtin(name: &str) -> Self {
        let mut widget = Widget::default();
        widget.name = name.to_string();
        widget.set_is_built_in(BuiltIn::try_from(name).is_ok());
        widget
    }
    pub fn set_id(&mut self, id: Option<&String>) -> &mut Self {
        if let Some(id) = id {
            self.id = Some(id.to_string());
        }
        self
    }
    pub fn set_as_prop(&mut self, as_prop: bool) -> &mut Self{
        self.as_prop = as_prop;
        self
    }
    pub fn set_is_root(&mut self, is_root: bool) -> &mut Self {
        self.is_root = is_root;
        self
    }
    pub fn set_is_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }
    pub fn set_is_prop(&mut self, is_prop: bool) -> &mut Self {
        self.is_prop = is_prop;
        self
    }
    pub fn set_is_built_in(&mut self, is_built_in: bool) -> &mut Self {
        self.is_built_in = is_built_in;
        self
    }
    /// if can not parse by BuiltIn Widget -> panic!
    pub fn set_props(&mut self, props: Option<HashMap<PropsKey, Value>>) -> &mut Self {
        if let Some(props) = props {
            // if self.is_built_in {
            //     self.props = Some(BuiltIn::from(&self.name).props(&props));
            // } else {
            //     todo!("widget props define unsoloved => {:#?}",props);
            // }
            if self.is_built_in {
                self.props = Some(BuiltIn::from(&self.name).props(&props));
            }
        }
        self
    }
    // pub fn push_prop(&mut self, key: String, value: TokenStream) -> &mut Self {
    //     if self.props.is_none() {
    //         self.props.replace(HashMap::new());
    //     }
    //     self.props.as_mut().unwrap().insert(key, value);

    //     self
    // }
    /// ## set widget script
    /// - set prop_ptr
    /// - set event_ptr
    /// - set uses
    /// - set draw_walk
    /// - set handle_event
    pub fn set_script(&mut self, script: Option<&ScriptModel>) -> &mut Self {
        if let Some(sc) = script {
            if let ScriptModel::Gen(sc) = sc {
                let GenScriptModel {
                    uses,
                    prop_ptr,
                    event_ptr,
                    sub_prop_binds,
                    sub_event_binds,
                    // other,
                    imports,
                    current_instance,
                    instance_opt,
                    ..
                } = sc;
                // 在这里从prop_ptr中获取结构体所有的field作为后续代码中需要转换的列表
                // 例如在handle_event中就需要
                let prop_fields = get_props_fields(prop_ptr.as_ref());
                // dbg!(prop_fields);

                self.set_uses(uses)
                    .set_imports(imports)
                    .set_prop_ptr(prop_ptr)
                    .set_event_ptr(event_ptr)
                    .after_apply(
                        sub_prop_binds,
                        current_instance.as_ref(),
                        instance_opt.as_ref(),
                    )
                    .draw_walk(None) // 暂时先写个None
                    .handle_event(
                        sub_prop_binds,
                        sub_event_binds,
                        current_instance.as_ref(),
                        prop_fields.as_ref(),
                    );
            }
        } else {
            self.is_static = true;
        }
        self
    }

    /// - prop_binds: 模板中绑定的props，用于对模板中的props进行更新，它能够跟踪到底prop应该如何更新
    /// - events: 模板中绑定的events
    /// - current_instance: 当前实例(属性实例)，用于获取实例名，它需要和prop_fields一起使用，来找到原Gen代码中需要被替换的部分(`current_instance.prop_field`)
    /// - prop_fields: 用于获取prop_ptr中的所有字段，用于在handle_event中找到需要更新的属性部分然后替换
    pub fn handle_event(
        &mut self,
        prop_binds: &Option<Vec<PropFn>>,
        events: &Option<Vec<PropFn>>,
        current_instance: Option<&CurrentInstance>,
        prop_fields: Option<&Vec<Ident>>,
    ) -> &mut Self {
        // dbg!(prop_fields);
        let instance_name = if let Some(instance) = current_instance {
            instance.name()
        } else {
            None
        };
        let builtin = self.inherits.as_ref().unwrap();
        let handle_event_tk = builtin.handle_event(events, prop_binds, instance_name, prop_fields);
        let _ = self.traits.as_mut().unwrap().handle_event(handle_event_tk);
        self
    }
    pub fn after_apply(
        &mut self,
        prop_binds: &Option<Vec<PropFn>>,
        current_instance: Option<&CurrentInstance>,
        instance_opt: Option<&Vec<Stmt>>,
    ) -> &mut Self {
        // 将当前实例所涉及的代码转为TokenStream
        // 需要将特定的头部转为self
        let apply_tk = instance_opt.map(|opt| {
            let instance_name = current_instance
                .unwrap()
                .name()
                .expect("current instance must have name")
                .to_string();
            opt.into_iter().fold(TokenStream::new(), |mut acc, item| {
                // 这里我本来可以一点点替换的，但发现似乎这样会错过很多情况，所以转而使用转为String后进行replace
                let item = item.to_token_stream().to_string();

                // let item = item.replacen(&instance_name, "self", 1);
                let item = item.replace(&instance_name, "self");

                acc.extend(parse_str::<TokenStream>(&item));
                acc
            })
        });

        let draw_widget_tk = quote_draw_widget(prop_binds);

        let apply_tk = combine_option(apply_tk, draw_widget_tk);

        let _ = self.live_hook.as_mut().unwrap().after_apply(apply_tk);

        self
    }
    pub fn draw_walk(&mut self, draw_walk_tk: Option<TokenStream>) -> &mut Self {
        // 由BuiltIn确定如何draw_walk
        let builtin = self.inherits.as_ref().unwrap();
        let _ = self
            .traits
            .as_mut()
            .unwrap()
            .draw_walk(builtin.draw_walk(&draw_walk_tk));
        self
    }
    pub fn set_uses(&mut self, uses: &Option<UseMod>) -> &mut Self {
        if let Some(uses) = uses {
            self.uses = WidgetHandler::uses(uses);
        }
        self
    }
    pub fn set_imports(&mut self, imports: &Option<StmtMacro>) -> &mut Self {
        if let Some(imports) = imports {
            // get mac tokens
            self.imports.replace(imports.mac.tokens.clone());
        }
        self
    }
    pub fn set_events(&mut self, events: HashMap<String, TokenStream>) -> &mut Self {
        self.events = Some(events);
        self
    }
    pub fn push_event(&mut self, key: String, value: TokenStream) -> &mut Self {
        if self.events.is_none() {
            self.events.replace(HashMap::new());
        }
        self.events.as_mut().unwrap().insert(key, value);

        self
    }
    pub fn get_inherits(&self) -> Option<&BuiltIn> {
        self.inherits.as_ref()
    }
    pub fn set_prop_ptr(&mut self, prop_ptr: &Option<ItemStruct>) -> &mut Self {
        if let Some(prop_ptr) = prop_ptr {
            self.prop_ptr.replace(WidgetHandler::prop_ptr(
                prop_ptr,
                self.get_inherits().unwrap(),
            ));
        }
        self
    }
    pub fn set_event_ptr(&mut self, event_ptr: &Option<ItemEnum>) -> &mut Self {
        if let Some(event_ptr) = event_ptr {
            self.event_ptr.replace(WidgetHandler::event_ptr(event_ptr));
        }
        self
    }
    pub fn set_children(&mut self, children: Vec<Widget>) -> &mut Self {
        self.children = Some(children);
        self
    }
    pub fn push_child(&mut self, child: Widget) -> &mut Self {
        if self.children.is_none() {
            self.children.replace(vec![]);
        }
        self.children.as_mut().unwrap().push(child);

        self
    }
    pub fn set_inherits(&mut self, inherits: BuiltIn) -> &mut Self {
        self.inherits = Some(inherits);
        self
    }
    pub fn set_traits(&mut self, traits: WidgetTrait) -> &mut Self {
        self.traits.replace(traits);
        self
    }
    pub fn set_role(&mut self, role: Role) -> &mut Self {
        self.role = role;
        self
    }
    pub fn widget_children_tree(&self) -> Option<TokenStream> {
        let mut tk = TokenStream::new();
        if let Some(children) = &self.children {
            for child in children {
                let Widget {
                    is_root,
                    is_prop,
                    is_built_in,
                    id,
                    as_prop,
                    name,
                    props,
                    ..
                } = child;

                let name = if *is_built_in {
                    snake_to_camel(name).unwrap()
                } else {
                    name.to_string()
                };

                tk.extend(component_render(
                    id.as_ref(),
                    *is_root,
                    *is_prop,
                    *as_prop,
                    &name,
                    props.clone(),
                    child.widget_children_tree(),
                ));
            }
            Some(tk)
        } else {
            None
        }
    }
}

impl ToLiveDesign for Widget {
    /// get widget tree
    fn widget_tree(&self) -> Option<TokenStream> {
        let mut tk = TokenStream::new();
        // props and children
        let mut props_children = self.props.clone().unwrap_or_default();
        props_children.extend(self.widget_children_tree().unwrap_or_default());

        let ui = if self.is_static {
            self.id
                .as_ref()
                .expect("root widget need id to get widget tree")
                .to_string()
        } else {
            self.source.as_ref().unwrap().source_name_lower()
        };

        tk.extend(special_struct(
            &ui,
            &self.name,
            Some(props_children),
            self.is_static,
        ));

        if tk.is_empty() {
            None
        } else {
            Some(tk)
        }
    }
    fn widget_logic(&self) -> Option<TokenStream> {
        if !self.is_static {
            let mut tk = TokenStream::new();
            if let Some(uses_tk) = &self.uses {
                tk.extend(uses_tk.clone());
            }
            if let Some(prop_ptr_tk) = &self.prop_ptr {
                tk.extend(prop_ptr_tk.clone());
            }
            if let Some(event_ptr_tk) = &self.event_ptr {
                tk.extend(event_ptr_tk.clone());
            }
            // tk.extend(
            //     self.traits
            //         .as_ref()
            //         .unwrap()
            //         .to_token_stream(token_tree_ident(&self.name)),
            // );
            if let Some(traits_tk) = &self.traits {
                tk.extend(traits_tk.to_token_stream(ident(&self.name)))
            }

            if let Some(event_set_tk) = &self.event_set {
                tk.extend(event_set_tk.clone());
            }
            if let Some(event_ref_tk) = &self.event_ref {
                tk.extend(event_ref_tk.clone());
            }
            if let Some(live_hook_tk) = &self.live_hook {
                tk.extend(live_hook_tk.to_token_stream(ident(&self.name)));
            }

            if tk.is_empty() {
                None
            } else {
                Some(tk)
            }
        } else {
            None
        }
    }
    fn widget_imports(&self) -> Option<TokenStream> {
        if let Some(imports) = self.imports.as_ref() {
            let imports = imports.to_string();
            let imports = imports
                .split(";")
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let tk = imports.iter().fold(TokenStream::new(), |mut acc, item| {
                let item: TokenStream = parse_str(item).unwrap();
                acc.extend(quote! {import #item;});
                acc
            });
            Some(tk)
        } else {
            None
        }
    }
    fn to_live_design(&self) -> super::live_design::LiveDesign {
        self.into()
    }
}

impl From<gen_converter::model::Model> for Widget {
    fn from(value: gen_converter::model::Model) -> Self {
        let gen_converter::model::Model {
            special,
            template,
            script,
            style,
            // compile,
            // is_entry,
            ..
        } = value;

        let template = template.unwrap();
        build_widget(Some(special), &template, style.as_ref(), script.as_ref())
    }
}

fn build_widget(
    special: Option<Source>,
    template: &TemplateModel,
    style: Option<&ConvertStyle>,
    script: Option<&ScriptModel>,
) -> Widget {
    let mut widget = Widget::new(special, template.get_name(), template.get_inherits());
    // get styles from style by id
    let widget_styles = get_widget_styles(template.get_id(), template.get_class(), style);
    let widget_styles = combine_styles(widget_styles, template.get_unbind_props());
    widget
        .set_is_root(template.is_root())
        .set_id(template.get_id())
        .set_as_prop(template.as_prop)
        .set_props(widget_styles)
        .set_script(script)
        .set_is_static(template.is_static());
    if template.has_children() {
        widget.set_children(
            template
                .get_children()
                .unwrap()
                .iter()
                .map(|item| build_widget(None, item, style, None))
                .collect(),
        );
    }
    return widget;
}

/// get styles from style by id
fn get_widget_styles(
    id: Option<&String>,
    class: Option<&Value>,
    styles: Option<&ConvertStyle>,
) -> Option<HashMap<PropsKey, Value>> {
    match styles {
        Some(styles) => {
            let mut map = HashMap::new();
            if let Some(id) = id {
                if let Some(id_styles) = styles.get(id) {
                    map.extend(id_styles.clone());
                }
            }
            if let Some(class) = class {
                if let Some(class_styles) = styles.get(class.to_string().as_str()) {
                    map.extend(class_styles.clone());
                }
            }
            if map.is_empty() {
                None
            } else {
                Some(map)
            }
            // match id {
            //     Some(id) => match styles.get(id) {
            //         Some(style) => Some(style.clone()),
            //         None => None,
            //     },
            //     None => None,
            // }
        }
        None => None,
    }
}

fn combine_styles(
    l: Option<HashMap<PropsKey, Value>>,
    r: Option<HashMap<&PropsKey, &Value>>,
) -> Option<HashMap<PropsKey, Value>> {
    match (l, r) {
        (Some(l), Some(r)) => {
            let mut styles = l.clone();
            for (k, v) in r {
                styles.insert(k.clone(), v.clone());
            }
            Some(styles)
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r.into_iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
        (None, None) => None,
    }
}

fn get_props_fields(prop_ptr: Option<&ItemStruct>) -> Option<Vec<Ident>> {
    if let Some(prop_ptr) = prop_ptr {
        let fields = prop_ptr
            .fields
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .collect();
        Some(fields)
    } else {
        None
    }
}
