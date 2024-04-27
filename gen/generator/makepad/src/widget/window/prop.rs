use std::fmt::Display;

use crate::widget::{view::ViewProps, StaticProps};

#[derive(Debug, Clone, Default)]
pub struct WindowProps(ViewProps);

impl  StaticProps for WindowProps{
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self where Self: Sized {
        WindowProps(ViewProps::props(props))
    }
}

impl Display for WindowProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}