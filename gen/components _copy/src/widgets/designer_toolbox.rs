use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    widgets::{designer_data::*, view::View, widget::*},
};

live_design! {
    DesignerToolboxBase = {{DesignerToolbox}}{
    }
}

#[derive(Live, Widget, LiveHook)]
pub struct DesignerToolbox {
    #[deref]
    view: View,
}

impl Widget for DesignerToolbox {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        let _data = scope.props.get::<DesignerData>().unwrap();
        while let Some(_next) = self.view.draw(cx, &mut Scope::empty()).step() {}
        DrawStep::done()
    }
}
