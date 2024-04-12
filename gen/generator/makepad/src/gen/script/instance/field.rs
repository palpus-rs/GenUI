use gen_utils::common::{token_tree_ident, token_tree_punct_alone, trees_to_token_stream};
use proc_macro2::TokenTree;

use crate::gen::ToToken;

use super::attr::Attr;

/// whole field: `#[live]|#[rust] [pub] name: ty,`
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    is_pub: bool,
    name: String,
    ty: String,
    // this attr is means #[live] | #[rust]
    attr: Attr,
}

#[allow(dead_code)]
impl Field {
    pub fn new(name: &str, ty: &str, attr: Attr) -> Self {
        Self {
            is_pub: true,
            name: name.to_string(),
            ty: ty.to_string(),
            attr,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &str {
        &self.ty
    }
    pub fn field_tk_only(&self)->Vec<TokenTree>{
        vec![
            token_tree_ident(&self.name),
            token_tree_punct_alone(','),
        ]
    }
}

impl ToToken for Field {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let mut tk = Vec::new();
        tk.extend(self.attr.to_token_stream());

        if self.is_pub {
            tk.push(token_tree_ident("pub"));
        }

        tk.push(token_tree_ident(&self.name));

        tk.push(token_tree_punct_alone(':'));
        
        tk.push(token_tree_ident(&self.ty));

        tk.push(token_tree_punct_alone(','));
        trees_to_token_stream(tk)
    }
}


#[cfg(test)]
mod field_test{
    use crate::gen::{script::instance::attr::Attr, ToToken};

    use super::Field;

    #[test]
    fn tk(){
        let field = Field::new("ui", "WidgetRef", Attr::Live);
        assert_eq!(field.to_token_stream().to_string().as_str(),"#[live] pub ui : WidgetRef ,")
    }
}