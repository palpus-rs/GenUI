use std::{collections::HashMap, fmt::Display};

use gen_parser::{PropsKey, Value};
use gen_utils::{
    error::Errors,
    props_manul::{self, Background, Cursor, Event, Others, Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{
            draw_color::DrawColor, Animation, EventOrder, Layout, MouseCursor, ViewOptimize, Walk,
        },
        ABS_POS, ALIGN, BLOCK_SIGNAL_EVENT, CLIP_X, CLIP_Y, COLOR, CURSOR, DRAW_BG, EVENT_ORDER,
        FLOW, GRAB_KEY_FOCUS, HEIGHT, LINE_SPACING, MARGIN, OPTIMIZE, PADDING, SCROLL, SHOW_BG,
        SPACING, VISIBLE, WIDTH,
    },
    widget::{
        prop_ignore, utils::{bind_prop_value, bool_prop, quote_prop}, AnimationApplys, BuiltIn, DynProps, StaticProps
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct ViewProps {
    pub draw_bg: Option<DrawColor>,
    pub show_bg: Option<bool>,
    pub layout: Option<Layout>,
    pub walk: Option<Walk>,
    pub optimize: Option<ViewOptimize>,
    pub event_order: Option<EventOrder>,
    pub visible: Option<bool>,
    pub grab_key_focus: Option<bool>,
    pub block_signal_event: Option<bool>,
    pub cursor: Option<MouseCursor>,
    pub animation: Option<Animation>,
}
impl DynProps for ViewProps {
    fn prop_bind(prop: &PropsKey, value: &Value, is_prop: bool, ident: &str) -> TokenStream {
        let value = bind_prop_value(value, is_prop, ident);

        match prop.name() {
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG, COLOR], &value),
            Background::BACKGROUND_VISIBLE => quote_prop(vec![SHOW_BG], &value),
            // ----------------- layout -----------------
            Others::SCROLL => quote_prop(vec![SCROLL], &value),
            Size::CLIP_X => quote_prop(vec![CLIP_X], &value),
            Size::CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            Size::PADDING => quote_prop(vec![PADDING], &value),
            Position::ALIGN => quote_prop(vec![ALIGN], &value),
            Position::FLOW => quote_prop(vec![FLOW], &value),
            Position::SPACING => quote_prop(vec![SPACING], &value),
            LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            // ----------------- other -----------------
            Others::OPTIMIZE => quote_prop(vec![OPTIMIZE], &value),
            Event::EVENT_ORDER => quote_prop(vec![EVENT_ORDER], &value),
            Others::VISIBLE => quote_prop(vec![VISIBLE], &value),
            Event::GRAB_KEY_FOCUS => quote_prop(vec![GRAB_KEY_FOCUS], &value),
            Event::BLOCK_SIGNAL_EVENT => quote_prop(vec![BLOCK_SIGNAL_EVENT], &value),
            Cursor::CURSOR => quote_prop(vec![CURSOR], &value),
            _ => panic!("cannot match prop"),
        }
    }
}

impl StaticProps for ViewProps {
    fn props(props: &HashMap<PropsKey, Value>) -> Self {
        let mut view = ViewProps::default();
        for (k, v) in props {
            view.prop(k.name(), v)
        }
        view
    }

    fn prop(&mut self, prop_name: &str, value: &Value) -> () {
        if prop_name.contains(props_manul::Animation::ANIMATION) {
            let _ = self.animation(prop_name, &value);
        } else {
            let _ = match prop_name {
                Background::BACKGROUND_COLOR => self.draw_bg(&value),
                Background::BACKGROUND_VISIBLE => self.show_bg(&value),
                // ----------------- layout -----------------
                Others::SCROLL => self.scroll(&value),
                Size::CLIP_X => self.clip_x(&value),
                Size::CLIP_Y => self.clip_y(&value),
                Size::PADDING => self.padding(&value),
                Position::ALIGN => self.align(&value),
                Position::FLOW => self.flow(&value),
                Position::SPACING => self.spacing(&value),
                LINE_SPACING => self.line_spacing(&value),
                // ----------------- walk -----------------
                Size::HEIGHT => self.height(&value),
                Size::WIDTH => self.width(&value),
                Position::ABS_POS => self.abs_pos(&value),
                Size::MARGIN => self.margin(&value),
                // ----------------- other -----------------
                Others::OPTIMIZE => self.optimize(&value),
                Event::EVENT_ORDER => self.event_order(&value),
                Others::VISIBLE => self.visible(&value),
                Event::GRAB_KEY_FOCUS => self.grab_key_focus(&value),
                Event::BLOCK_SIGNAL_EVENT => self.block_signal_event(&value),
                Cursor::CURSOR => self.mouse_cursor(&value),
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
}

impl ToToken for ViewProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl AnimationApplys for ViewProps {
    fn animation_applys() -> Vec<&'static str> {
        vec!["draw_bg"]
    }
}

impl ViewProps {
    fn animation(&mut self, prop_name: &str, value: &Value) -> Result<(), Errors> {
        let apply_event = prop_name.split("::").collect::<Vec<&str>>()[1];
        if let Some(an) = self.animation.as_mut() {
            an.push((apply_event, value, BuiltIn::View).try_into()?)
        } else {
            self.animation = Some((apply_event, value, BuiltIn::View).try_into()?);
        };
        Ok(())
    }
   
    fn show_bg(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.show_bg = Some(b);
        })
    }
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        let color = DrawColor::try_from(value)?;
        self.draw_bg = Some(color);
        Ok(())
    }
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn height(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().height(value)
    }
    fn width(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().width(value)
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn visible(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.visible = Some(b);
        })
    }
    fn grab_key_focus(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.grab_key_focus = Some(b);
        })
    }
    fn block_signal_event(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.block_signal_event = Some(b);
        })
    }
    fn optimize(&mut self, value: &Value) -> Result<(), Errors> {
        self.optimize = Some(value.try_into()?);
        Ok(())
    }
    fn mouse_cursor(&mut self, value: &Value) -> Result<(), Errors> {
        self.cursor = Some(value.try_into()?);
        Ok(())
    }
    fn event_order(&mut self, value: &Value) -> Result<(), Errors> {
        self.event_order = Some(value.try_into()?);
        Ok(())
    }
    fn abs_pos(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().abs_pos(value)
    }
    fn margin(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_walk().margin(value)
    }
    fn check_layout(&mut self) -> &mut Layout {
        if self.layout.is_none() {
            self.layout = Some(Layout::default());
        }
        self.layout.as_mut().unwrap()
    }
    fn scroll(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().scroll(value)
    }
    fn clip_x(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_x(value)
    }
    fn clip_y(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_y(value)
    }
    fn padding(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().padding(value)
    }
    fn flow(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().flow(value)
    }
    fn spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().spacing(value)
    }
    fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().line_spacing(value)
    }
}

impl Display for ViewProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = self.draw_bg.as_ref() {
            let _ = f.write_fmt(format_args!("draw_bg: {{{}}}, ", draw_bg));
        }
        if let Some(show_bg) = self.show_bg.as_ref() {
            let _ = f.write_fmt(format_args!("show_bg: {}, ", show_bg));
        }
        if let Some(layout) = self.layout.as_ref() {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(walk) = self.walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(optimize) = self.optimize.as_ref() {
            let _ = f.write_fmt(format_args!("optimize: {}, ", optimize));
        }
        if let Some(event_order) = self.event_order.as_ref() {
            let _ = f.write_fmt(format_args!("event_order: {}, ", event_order));
        }
        if let Some(visible) = self.visible.as_ref() {
            let _ = f.write_fmt(format_args!("visible: {}, ", visible));
        }
        if let Some(grab_key_focus) = self.grab_key_focus.as_ref() {
            let _ = f.write_fmt(format_args!("grab_key_focus: {}, ", grab_key_focus));
        }
        if let Some(block_signal_event) = self.block_signal_event.as_ref() {
            let _ = f.write_fmt(format_args!("block_signal_event: {}, ", block_signal_event));
        }
        if let Some(cursor) = self.cursor.as_ref() {
            let _ = f.write_fmt(format_args!("cursor: {}, ", cursor));
        }
        if let Some(animation) = self.animation.as_ref() {
            let _ = f.write_fmt(format_args!("animator: {{{}}}, ", animation));
        }
        f.write_str("")
    }
}

#[cfg(test)]
mod test_view_props {

    use super::*;
    #[test]
    fn to_tk() {
        let mut view = ViewProps::default();
        view.block_signal_event = Some(true);
        view.draw_bg = Some(DrawColor::default());
        view.show_bg = Some(true);
        let mut layout = Layout::default();
        layout.spacing = Some(10_f64);
        layout.line_spacing = Some(1.5_f64);
        layout.clip_x = Some(true);
        layout.clip_y = Some(false);
        layout.padding = Some("4 10".try_into().unwrap());
        layout.align = Some("0.5 1".try_into().unwrap());
        layout.flow = Some("Down".try_into().unwrap());
        layout.scroll = Some("1 2".try_into().unwrap());
        view.layout = Some(layout);
        let mut walk = super::Walk::default();
        walk.abs_pos = Some("10 10".try_into().unwrap());
        walk.margin = Some("10 10 10 10".try_into().unwrap());
        walk.width = Some("100".try_into().unwrap());
        walk.height = Some("100".try_into().unwrap());
        view.walk = Some(walk);
        view.optimize = Some(ViewOptimize::default());
        view.event_order = Some(EventOrder::default());
        view.visible = Some(true);
        view.grab_key_focus = Some(true);
        view.cursor = Some(MouseCursor::Hand);
        let tk = view.to_token_stream();
        let prop = "draw_bg : { ## 000 } , show_bg : true , scroll : { x : 1 , y : 2 } , clip_x : true , clip_y : false , padding : { top : 10 , right : 4 , bottom : 10 , left : 4 } , align : { x : 0.5 , y : 1 } , flow : Down , spacing : 10 , line_spacing : 1.5 , abs_pos : { x : 10 , y : 10 } , margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , width : 100 , height : 100 , optimize : None , event_order : Up , visible : true , grab_key_focus : true , block_signal_event : true , cursor : Hand ,";

        assert_eq!(prop, tk.to_string().as_str());
    }
}
