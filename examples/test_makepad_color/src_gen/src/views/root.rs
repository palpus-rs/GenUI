use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; ui = <Root >{ main_window = < Window >{ window : { position : vec2 (300 , 300) , } draw_bg : { color : # 1C2128FF } , show_bg : true , flow : Down , width : Fill , height : Fill , view1 = < View >{ draw_bg : { color : # FFFFFFFF } , width : 100 , height : 100 , } } } }