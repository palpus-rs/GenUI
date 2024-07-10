use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: views :: root :: * ; App = { { App } } { root : < ui > { } } } # [derive (Live , LiveHook)] pub struct App { #[live] pub root : WidgetRef , } impl MatchEvent for App { } impl AppMain for App { fn handle_event (& mut self , cx : & mut Cx , event : & Event) { self . match_event (cx , event) ; self . root . handle_event (cx , event , & mut Scope :: empty ()) ; } } impl LiveRegister for App { fn live_register (cx : & mut Cx) { crate :: makepad_widgets :: live_design (cx) ; crate :: views :: root :: live_design (cx) ; crate :: views :: components :: button_view :: live_design (cx) ; crate :: views :: components :: checkbox :: live_design (cx) ; crate :: views :: components :: drop_down_view :: live_design (cx) ; crate :: views :: components :: labels :: live_design (cx) ; } } app_main ! (App) ;