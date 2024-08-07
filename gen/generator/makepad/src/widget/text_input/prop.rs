#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Background, Cursor, Font, Others, Position, Size, State, Text},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{
            draw_color::DrawColor, draw_label::DrawLabel, draw_quad::DrawQuad, Align, Layout, Walk,
        },
        ABS_POS, ALIGN, ASCII_ONLY, BRIGHTNESS, CLIP_X, CLIP_Y, COLOR, COMBINE_SPACES,
        CURSOR_MARGIN_BOTTOM, CURSOR_MARGIN_TOP, CURSOR_SIZE, CURVE, DRAW_BG, DRAW_CURSOR,
        DRAW_DEPTH, DRAW_SELECT, DRAW_TEXT, EMPTY_MESSAGE, FLOW, FONT, FONT_SCALE, FONT_SIZE,
        HEIGHT, HEIGHT_FACTOR, INGORE_NEWLINES, IS_EMPTY, LABEL_ALIGN, LINE_SPACING, MARGIN,
        NUMERIC_ONLY, ON_FOCUS_SELECT_ALL, PADDING, READ_ONLY, SCROLL, SECRET, SELECT_PAD_EDGES,
        SPACING, TEXT, TOP_DROP, WIDTH, WRAP,
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, bool_prop, f64_prop, quote_prop, string_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct TextInputProps {
    pub draw_bg: Option<DrawColor>,
    pub draw_select: Option<DrawQuad>,
    pub draw_cursor: Option<DrawQuad>,
    pub draw_text: Option<DrawLabel>,
    pub walk: Option<Walk>,
    pub layout: Option<Layout>,
    pub label_align: Option<Align>,
    pub cursor_size: Option<f64>,
    pub cursor_margin_bottom: Option<f64>,
    pub cursor_margin_top: Option<f64>,
    pub select_pad_edges: Option<f64>,
    pub empty_message: Option<String>,
    pub numeric_only: Option<bool>,
    pub secret: Option<bool>,
    pub on_focus_select_all: Option<bool>,
    pub read_only: Option<bool>,
    pub text: Option<String>,
    pub ascii_only: Option<bool>,
}

impl DynProps for TextInputProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- draw_bg ---------------
            Background::BACKGROUND_COLOR => quote_prop(vec![DRAW_BG, COLOR], &value),
            // ----------------- draw_select ---------------
            DRAW_SELECT => quote_prop(vec![DRAW_SELECT], &value),
            // ----------------- draw_cursor ---------------
            DRAW_CURSOR => quote_prop(vec![DRAW_CURSOR], &value),
            // ----------------- draw_text -----------------
            Font::FONT_FAMILY => quote_prop(vec![DRAW_TEXT, FONT], &value),
            Font::FONT_SIZE => quote_prop(vec![DRAW_TEXT, FONT_SIZE], &value),
            Font::BRIGHTNESS => quote_prop(vec![DRAW_TEXT, BRIGHTNESS], &value),
            Font::CURVE => quote_prop(vec![DRAW_TEXT, CURVE], &value),
            "text_line_spacing" => quote_prop(vec![DRAW_TEXT, LINE_SPACING], &value),
            Font::TOP_DROP => quote_prop(vec![DRAW_TEXT, TOP_DROP], &value),
            Font::HEIGHT_FACTOR => quote_prop(vec![DRAW_TEXT, HEIGHT_FACTOR], &value),
            Text::TEXT_WRAP => quote_prop(vec![DRAW_TEXT, WRAP], &value),
            Text::IGNORE_NEWLINES => quote_prop(vec![DRAW_TEXT, INGORE_NEWLINES], &value),
            Text::COMBINE_SPACES => quote_prop(vec![DRAW_TEXT, COMBINE_SPACES], &value),
            Font::FONT_SCALE => quote_prop(vec![DRAW_TEXT, FONT_SCALE], &value),
            Text::DRAW_DEPTH => quote_prop(vec![DRAW_TEXT, DRAW_DEPTH], &value),
            Text::COLOR => quote_prop(vec![DRAW_TEXT, COLOR], &value),
            Text::EMPTY => quote_prop(vec![DRAW_TEXT, IS_EMPTY], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            // ------------------- layout -----------------
            Others::SCROLL => quote_prop(vec![SCROLL], &value),
            Size::CLIP_X => quote_prop(vec![CLIP_X], &value),
            Size::CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            Size::PADDING => quote_prop(vec![PADDING], &value),
            Position::ALIGN => quote_prop(vec![ALIGN], &value),
            Position::FLOW => quote_prop(vec![FLOW], &value),
            Position::SPACING => quote_prop(vec![SPACING], &value),
            LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            // ----------------- other ------------------
            Text::TEXT_ALIGN => quote_prop(vec![LABEL_ALIGN], &value),
            Cursor::CURSOR_SIZE => quote_prop(vec![CURSOR_SIZE], &value),
            Cursor::CURSOR_MARGIN_BOTTOM => quote_prop(vec![CURSOR_MARGIN_BOTTOM], &value),
            Cursor::CURSOR_MARGIN_TOP => quote_prop(vec![CURSOR_MARGIN_TOP], &value),
            Others::SELECT_PAD_EDGES => quote_prop(vec![SELECT_PAD_EDGES], &value),
            Text::EMPTY_MESSAGE => quote_prop(vec![EMPTY_MESSAGE], &value),
            State::NUMERIC_ONLY => quote_prop(vec![NUMERIC_ONLY], &value),
            State::SECRET => quote_prop(vec![SECRET], &value),
            Others::ON_FOCUS_SELECT_ALL => quote_prop(vec![ON_FOCUS_SELECT_ALL], &value),
            State::READ_ONLY => quote_prop(vec![READ_ONLY], &value),
            Text::TEXT => quote_prop(vec![TEXT], &value),
            State::ASCII_ONLY => quote_prop(vec![ASCII_ONLY], &value),
            _ => panic!("cannot match prop in BuiltIn Icon"),
        }
    }
}

impl StaticProps for TextInputProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut radio = TextInputProps::default();
        for (k, v) in props {
            radio.prop(k.name(), v)
        }
        radio
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- draw_bg ---------------
            Background::BACKGROUND_COLOR => self.draw_bg(&value),
            // ----------------- draw_select ---------------
            DRAW_SELECT => self.draw_select(&value),
            // ----------------- draw_cursor ---------------
            DRAW_CURSOR => self.draw_cursor(&value),
            // ----------------- draw_text -----------------
            Font::FONT_FAMILY => self.font(&value),
            Font::FONT_SIZE => self.font_size(&value),
            Font::BRIGHTNESS => self.brightness(&value),
            Font::CURVE => self.curve(&value),
            "text_line_spacing" => self.label_line_spacing(&value),
            Font::TOP_DROP => self.top_drop(&value),
            Font::HEIGHT_FACTOR => self.height_factor(&value),
            Text::TEXT_WRAP => self.wrap(&value),
            Text::IGNORE_NEWLINES => self.ignore_newlines(&value),
            Text::COMBINE_SPACES => self.combine_spaces(&value),
            Font::FONT_SCALE => self.font_scale(&value),
            Text::DRAW_DEPTH => self.draw_depth(&value),
            Text::COLOR => self.color(&value),
            Text::EMPTY => self.is_empty(&value),
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value),
            Size::WIDTH => self.width(&value),
            Position::ABS_POS => self.abs_pos(&value),
            Size::MARGIN => self.margin(&value),
            // ------------------- layout -----------------
            Others::SCROLL => self.scroll(&value),
            Size::CLIP_X => self.clip_x(&value),
            Size::CLIP_Y => self.clip_y(&value),
            Size::PADDING => self.padding(&value),
            Position::ALIGN => self.align(&value),
            Position::FLOW => self.flow(&value),
            Position::SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value),
            // ----------------- other ------------------
            Text::TEXT_ALIGN => self.label_align(&value),
            Cursor::CURSOR_SIZE => self.cursor_size(&value),
            Cursor::CURSOR_MARGIN_BOTTOM => self.cursor_margin_bottom(&value),
            Cursor::CURSOR_MARGIN_TOP => self.cursor_margin_top(&value),
            Others::SELECT_PAD_EDGES => self.select_pad_edges(&value),
            Text::EMPTY_MESSAGE => self.empty_message(&value),
            State::NUMERIC_ONLY => self.numeric_only(&value),
            State::SECRET => self.secret(&value),
            Others::ON_FOCUS_SELECT_ALL => self.on_focus_select_all(&value),
            State::READ_ONLY => self.read_only(&value),
            Text::TEXT => self.text(&value),
            State::ASCII_ONLY => self.ascii_only(&value),
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
impl TextInputProps {
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_bg = Some(DrawColor::try_from(value).unwrap());
        Ok(())
    }
    fn draw_select(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_select = Some(DrawQuad::try_from(value).unwrap());
        Ok(())
    }
    fn draw_cursor(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_cursor = Some(DrawQuad::try_from(value).unwrap());
        Ok(())
    }
    fn check_draw_text(&mut self) -> &mut DrawLabel {
        if self.draw_text.is_none() {
            self.draw_text = Some(DrawLabel::default());
        }
        self.draw_text.as_mut().unwrap()
    }
    fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.font(value)
    }
    fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.font_size(value)
    }
    fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.brightness(value)
    }
    fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.curve(value)
    }
    fn label_line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.line_spacing(value)
    }
    fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.top_drop(value)
    }
    fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.height_factor(value)
    }
    fn wrap(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.wrap(value)
    }
    fn ignore_newlines(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.ignore_newlines(value)
    }
    fn combine_spaces(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.combine_spaces(value)
    }
    fn font_scale(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.font_scale(value)
    }
    fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.draw_depth(value)
    }
    fn color(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().draw_super.color(value)
    }
    fn is_empty(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().is_empty(value)
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
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn label_align(&mut self, value: &Value) -> Result<(), Errors> {
        self.label_align = Some(Align::try_from(value).unwrap());
        Ok(())
    }
    fn cursor_size(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.cursor_size.replace(f);
        })
    }
    fn cursor_margin_bottom(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.cursor_margin_bottom.replace(f);
        })
    }
    fn cursor_margin_top(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.cursor_margin_top.replace(f);
        })
    }
    fn select_pad_edges(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.select_pad_edges.replace(f);
        })
    }
    fn empty_message(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            self.empty_message.replace(s.to_string());
        })
    }
    fn numeric_only(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.numeric_only.replace(b);
        })
    }
    fn secret(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.secret.replace(b);
        })
    }
    fn on_focus_select_all(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.on_focus_select_all.replace(b);
        })
    }
    fn read_only(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.read_only.replace(b);
        })
    }
    fn text(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            self.text.replace(s.to_string());
        })
    }
    fn ascii_only(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.ascii_only.replace(b);
        })
    }
}

impl Display for TextInputProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = self.draw_bg.as_ref() {
            let _ = f.write_fmt(format_args!("{}: {{{}}}", DRAW_BG, draw_bg));
        }
        if let Some(draw_select) = &self.draw_select {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_SELECT, draw_select));
        }
        if let Some(draw_cursor) = &self.draw_cursor {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_CURSOR, draw_cursor));
        }
        if let Some(draw_text) = &self.draw_text {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_TEXT, draw_text));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(layout) = &self.layout {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        if let Some(label_align) = self.label_align.as_ref() {
            let _ = f.write_fmt(format_args!("label_align: {},", label_align));
        }
        if let Some(cursor_size) = self.cursor_size {
            let _ = f.write_fmt(format_args!("cursor_size: {},", cursor_size));
        }
        if let Some(cursor_margin_bottom) = self.cursor_margin_bottom {
            let _ = f.write_fmt(format_args!(
                "cursor_margin_bottom: {},",
                cursor_margin_bottom
            ));
        }
        if let Some(cursor_margin_top) = self.cursor_margin_top {
            let _ = f.write_fmt(format_args!("cursor_margin_top: {},", cursor_margin_top));
        }
        if let Some(select_pad_edges) = self.select_pad_edges {
            let _ = f.write_fmt(format_args!("select_pad_edges: {},", select_pad_edges));
        }
        if let Some(empty_message) = self.empty_message.as_ref() {
            let _ = f.write_fmt(format_args!("empty_message: {},", empty_message));
        }
        if let Some(numeric_only) = self.numeric_only {
            let _ = f.write_fmt(format_args!("numeric_only: {},", numeric_only));
        }
        if let Some(secret) = self.secret {
            let _ = f.write_fmt(format_args!("secret: {},", secret));
        }
        if let Some(on_focus_select_all) = self.on_focus_select_all {
            let _ = f.write_fmt(format_args!(
                "on_focus_select_all: {},",
                on_focus_select_all
            ));
        }
        if let Some(read_only) = self.read_only {
            let _ = f.write_fmt(format_args!("read_only: {},", read_only));
        }
        if let Some(text) = self.text.as_ref() {
            let _ = f.write_fmt(format_args!("text: \"{}\",", text));
        }
        if let Some(ascii_only) = self.ascii_only {
            let _ = f.write_fmt(format_args!("ascii_only: {},", ascii_only));
        }
        write!(f, "")
    }
}

props_to_token!(TextInputProps);
