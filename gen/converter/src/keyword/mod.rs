use std::fmt::Display;

use gen_parser::Value;

use crate::model::{Model, TemplateModel};

const PROPS: &str = "props";
const ID: &str = "id";
const CLASS: &str = "class";
const INHERITS: &str = "inherits";
const ACTIONS_MACRO: &str = "actions!";

/// The key words in gen-ui template
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum KeyWords {
    /// :props
    Props,
    /// id
    Id,
    /// class
    Class,
    /// inherits
    Inherits,
    Actions_Macro,
}

impl KeyWords {
    pub fn value_prop(&self, value: &Value, model: &mut TemplateModel) -> () {
        match self {
            KeyWords::Props => {
                // props只能是绑定的
                let props = value.is_bind_and_get().unwrap();
                // model.push_prop(item)
            }
            KeyWords::Id => {
                // id只能是单个String或Unknown
                // if let Some(id) = value.is_unknown_and_get() {
                //     let _ = model.set_special(id);
                // } else {
                //     value.is_string_and_get().unwrap_or_else(|s| {
                //         let _ = model.set_special(s);
                //     });
                // }
                string_unknown(value, |id| {
                    model.set_special(id);
                });
            }
            KeyWords::Class => {
                // class没有限制，可以是String,Unknown,绑定
            }
            KeyWords::Inherits => {
                // inherits只能是单个String或Unknown
                string_unknown(value, |inherits| model.set_inherit(inherits));
            }
            _ => panic!("KeyWord can not use in Template prop"),
        }
    }
}
impl Display for KeyWords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            KeyWords::Props => PROPS,
            KeyWords::Id => ID,
            KeyWords::Class => CLASS,
            KeyWords::Inherits => INHERITS,
            KeyWords::Actions_Macro => ACTIONS_MACRO,
        })
    }
}

impl TryFrom<&str> for KeyWords {
    type Error = crate::error::Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            PROPS => Ok(KeyWords::Props),
            ID => Ok(KeyWords::Id),
            CLASS => Ok(KeyWords::Class),
            INHERITS => Ok(KeyWords::Inherits),
            ACTIONS_MACRO => Ok(KeyWords::Actions_Macro),
            _ => Err(crate::error::Errors::MissMatchKeyWord),
        }
    }
}

fn string_unknown<F>(value: &Value, f: F) -> ()
where
    F: FnOnce(&str) -> (),
{
    if let Some(id) = value.is_unknown_and_get() {
        let _ = f(id);
    } else {
        value.is_string_and_get().map(|id| {
            let _ = f(id);
        });
    }
}