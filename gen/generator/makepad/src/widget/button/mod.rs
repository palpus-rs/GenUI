use gen_parser::PropsKey;
use gen_utils::common::{token_stream_to_tree, token_tree_ident, trees_to_token_stream};
use proc_macro2::{Group, TokenStream, TokenTree};

use crate::{gen::FieldTable, utils::self_event_react};

pub fn event(
    root: Option<String>,
    id: String,
    pv: (PropsKey, String, TokenStream),
    field_table: &FieldTable,
) -> Vec<TokenTree> {
    let (ep, _, code) = pv;

    match ep.name() {
        "clicked" => button_clicked(root, id, "clicked", code, field_table),
        _ => panic!("not found event in button"),
    }
}

fn button_clicked(
    root: Option<String>,
    id: String,
    ident: &str,
    code: TokenStream,
    field_table: &FieldTable,
) -> Vec<TokenTree> {
    // 1. 获取field_table中的fields 并且遍历code中的节点，发现有field_table中的field则替换为field_table的prefix + field
    let prefix = field_table.self_prefix();
    let fields = field_table
        .get_fields()
        .iter()
        .filter(|item| {
            if let TokenTree::Ident(_) = item {
                return true;
            }
            false
        })
        .map(|item| {
            return if let TokenTree::Ident(ident) = item {
                ident.to_string()
            } else {
                panic!("field_table中的field必须是Ident类型{:#?}", item)
            };
        })
        .collect::<Vec<String>>();
    let visitor = EventVisitor::new(prefix, fields);

    let code = visitor.visit(token_stream_to_tree(code));

    // 2. 调用self_event_react方法构造

    let mut tk = vec![token_tree_ident("if")];
    tk.extend(self_event_react(root, "button", &id, ident, code));
    tk
}

struct EventVisitor {
    replace: TokenStream,
    fields: Vec<String>,
}

impl EventVisitor {
    pub fn new(replace: TokenStream, fields: Vec<String>) -> Self {
        Self { replace, fields }
    }
    fn visit(&self, target: Vec<TokenTree>) -> Vec<TokenTree> {
        let mut res = target.clone();
        let mut indexs = Vec::new();
        target.iter().enumerate().for_each(|(index, item)| {
            if let TokenTree::Group(group) = item {
                let handled = self.visit(token_stream_to_tree(group.stream()));
                res[index] = TokenTree::Group(Group::new(
                    group.delimiter(),
                    trees_to_token_stream(handled),
                ));
            }
            if let TokenTree::Ident(ident) = item {
                if self.fields.contains(&ident.to_string()) {
                    // 收集需要更改的索引
                    indexs.push(index);
                }
            }
        });
        for index in indexs.iter().rev() {
            res.splice(*index..*index, self.replace.clone());
        }
        res
    }
}
