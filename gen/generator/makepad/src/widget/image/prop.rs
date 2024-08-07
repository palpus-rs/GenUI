use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Background, Position, Resource, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_quad::DrawQuad, ImageFit, LiveDependency, Walk},
        ABS_POS, DRAW_BG, FIT, HEIGHT, MARGIN, MIN_HEIGHT, MIN_WIDTH, SOURCE, WIDTH, WIDTH_SCALE,
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, f64_prop, i64_prop, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct ImageProps {
    pub walk: Option<Walk>,
    pub draw_bg: Option<DrawQuad>,
    pub min_height: Option<i64>,
    pub min_width: Option<i64>,
    pub width_scale: Option<f64>,
    pub fit: Option<ImageFit>,
    pub source: Option<LiveDependency>,
    // todo!(texture: Option<Texture>)
}

impl DynProps for ImageProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            // ----------------- other -----------------
            Size::MIN_HEIGHT => quote_prop(vec![MIN_HEIGHT], &value),
            Size::MIN_WIDTH => quote_prop(vec![MIN_WIDTH], &value),
            Resource::SOURCE => quote_prop(vec![SOURCE], &value),
            Resource::FIT => quote_prop(vec![FIT], &value),
            Size::SCALE => quote_prop(vec![WIDTH_SCALE], &value),
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG], &value),
            _ => panic!("cannot match prop in BuiltIn Icon"),
        }
    }
}

impl StaticProps for ImageProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = ImageProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value),
            Size::WIDTH => self.width(&value),
            Position::ABS_POS => self.abs_pos(&value),
            Size::MARGIN => self.margin(&value),
            // ----------------- other -----------------
            Size::MIN_HEIGHT => self.min_height(&value),
            Size::MIN_WIDTH => self.min_width(&value),
            Resource::SOURCE => self.source(&value),
            Resource::FIT => self.fit(&value),
            Size::SCALE => self.width_scale(&value),
            Background::BACKGROUND_COLOR => self.draw_bg(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop: {}", prop_name);
                } else {
                    panic!("unslolved prop: {}", prop_name);
                }
            }
        };
    }
}

#[allow(dead_code)]
impl ImageProps {
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        if self.draw_bg.is_none() {
            self.draw_bg = Some(DrawQuad::default());
        }
        self.draw_bg.as_mut().unwrap().pixel(value)
    }
    fn height(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().height(value)
    }
    fn width(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().width(value)
    }
    fn abs_pos(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().abs_pos(value)
    }
    fn margin(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().margin(value)
    }
    fn min_height(&mut self, value: &Value) -> Result<(), Errors> {
        i64_prop(value, |int| {
            self.min_height = Some(int);
        })
    }
    fn min_width(&mut self, value: &Value) -> Result<(), Errors> {
        i64_prop(value, |int| {
            self.min_width = Some(int);
        })
    }
    fn fit(&mut self, value: &Value) -> Result<(), Errors> {
        let fit = ImageFit::try_from(value)?;
        self.fit = Some(fit);
        Ok(())
    }
    fn source(&mut self, value: &Value) -> Result<(), Errors> {
        self.source = Some(LiveDependency::try_from(value)?);
        Ok(())
    }
    fn width_scale(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.width_scale = Some(f);
        })
    }
}

impl Display for ImageProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!(DrawQuard pixel())
        if let Some(draw_bg) = &self.draw_bg {
            let _ = f.write_fmt(format_args!("{}: {{{}}}", DRAW_BG, draw_bg));
        }

        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(min_height) = self.min_height {
            let _ = f.write_fmt(format_args!("{}: {},", MIN_HEIGHT, min_height));
        }
        if let Some(min_width) = self.min_width {
            let _ = f.write_fmt(format_args!("{}: {},", MIN_WIDTH, min_width));
        }
        if let Some(fit) = &self.fit {
            let _ = f.write_fmt(format_args!("{}: {},", FIT, fit));
        }
        if let Some(source) = &self.source {
            let _ = f.write_fmt(format_args!("{}: {},", SOURCE, source));
        }
        if let Some(width_scale) = self.width_scale {
            let _ = f.write_fmt(format_args!("{}: {},", WIDTH_SCALE, width_scale));
        }
        write!(f, "")
    }
}

props_to_token!(ImageProps);
