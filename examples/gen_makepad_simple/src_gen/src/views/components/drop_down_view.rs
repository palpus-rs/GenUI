use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; drop_down_view = <View >{ align : { x : 0.5 , y : 0.5 } , width : Fill , height : 200 , drop1 = < DropDown >{ labels : ["Shanghai" , "Cake" , "Cow"] values : [1 , 2 , 3] } < DesktopButton >{ draw_bg : { button_type : WindowsMin , } } < Splitter >{ align : FromA (50.0) , a : < View >{ draw_bg : { color : vec4 (1.0 , 0.0 , 0.0 , 1.0) } , show_bg : true , width : 100 , } b : < View >{ draw_bg : { color : vec4 (1.0 , 0.0 , 1.0 , 1.0) } , show_bg : true , width : 100 , } } < RotatedImage >{ draw_bg : { opacity : 0.6 , rotation : 1.5707963267948966 , } width : 60 , height : 60 , source : dep ("crate://self/resources/robius.png") , } } }