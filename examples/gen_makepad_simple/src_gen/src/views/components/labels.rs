use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; label_view = <View >{ flow : Down , height : 140 , label1 = < Label >{ draw_text : { text_style : { font_size : 14 , brightness : 1.2 , curve : 1.5 , line_spacing : 1.5 , top_drop : 1.5 , } , combine_spaces : true , } , margin : { top : 10 , right : 16 , bottom : 10 , left : 16 } , width : 120 , height : 32 , padding : { top : 6 , right : 6 , bottom : 6 , left : 6 } , text : "This is Label 1" , } link1 = < LinkLabel >{ draw_text : { text_style : { font_size : 10 , } , } , text : "Gosim Example" , url : "https://gen.ipter.org/" , open_in_place : true , } } }