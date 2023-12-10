use makepad_widgets::*;
use crate::home::home_screen::*;
use crate::more::more_screen::*;
use crate::shared::stack_navigation::*;
use crate::shared::stack_view_action::StackViewAction;
//use crate::wasmedge_add::*;
use crate::wasmedge_async::*;
use std::collections::HashMap;
use tokio::runtime;
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::home::home_screen::HomeScreen;
    import crate::more::more_screen::MoreScreen;
    import crate::shared::stack_navigation::*;
    VIDEO_1 = dep("crate://self/resources/video_1.mp4")
    ICON_HOME = dep("crate://self/resources/icons/icon_home.svg")
    H3_TEXT_REGULAR = {
        font_size: 9.0,
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }
    AppTab = <RadioButton> {
        width: Fit,
        height: Fill,
        align: {x: 0.0, y: 0.0}
        draw_radio: {
            radio_type: Tab,
            color_active: #fff,
            color_inactive: #fff,
        }
        draw_text: {
            color_selected: #0b0,
            color_unselected: #000,
            color_unselected_hover: #111,
            text_style: <H3_TEXT_REGULAR> {}
        }
    }
    App = {{App}} {

        ui: <Window>{
            show_bg: true
            width: Fill,
            height: Fill
            
            draw_bg: {
                fn pixel(self) -> vec4 {
                    //return #000
                    return mix(#7, #3, self.pos.y);
                }
            }
            
            body = <View>{
                 
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                },
                <TextInput> {
                    align: {y: 0.7}
                    margin: {left: 5., top: 6.}
                    width: 100, height: 30,
                    text: "zzz2mn"
                }
                // button0 = <Button> {
                //     text: "Get model"
                // }
                model_label = <Label> {
                    width: 300,
                    height: Fit
                    draw_text: {
                        color: #f
                    },
                    text: "",
                }
                
                message_input = <TextInput> {
                    text: "Hi, what is the capital of america?"
                    width: 300,
                    height: Fit
                    draw_bg: {
                        color: #1
                    }
                }
                button1 = <Button> {
                    text: "Send"
                }
                
                label1 = <Label> {
                    draw_text: {
                        color: #f
                    },
                    text: "Reply: --"
                }
                
                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill,
                        height: Fill,
                        padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down

                        application_pages = <View> {
                            margin: 0.0,
                            padding: 0.0
                            tab1_frame = <HomeScreen> {visible: true}
                            tab2_frame = <MoreScreen> {visible: false}
                        }
                    }
                }
                mobile_menu = <RoundedView> {
                    width: Fill,
                    height: 80,
                    flow: Right, spacing: 6.0, padding: 10
                    draw_bg: {
                        instance radius: 0.0,
                        instance border_width: 1.0,
                        instance border_color: #aaa,
                        //color: #fff
                    }

                    mobile_modes = <View> {
                        tab1 = <AppTab> {
                            animator: {selected = {default: on}}
                            label: "Home"
                            draw_icon: {
                                svg_file: (ICON_HOME),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            width: Fill,
                            icon_walk: {width: 20, height: 20}
                            flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
                        }
                        tab2 = <AppTab> {
                            label: "More"
                            draw_icon: {
                                svg_file: (ICON_HOME),
                                fn get_color(self) -> vec4 {
                                    return mix(
                                        #000,
                                        #0b0,
                                        self.selected
                                    )
                                }
                            }
                            width: Fill,
                            icon_walk: {width: 20, height: 20}
                            flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
                        }
                    }
                }
                home_view = <StackNavigationView> {
                    header = {
                        content = {
                            title_container = {
                                title = {
                                    text: "Home Sccccreeen"
                                }
                            }
                        }
                    }
                    <TextInput> {
                        align: {y: 0.7}
                        margin: {left: 5., top: 6.}
                        width: 100, height: 30,
                        text: "home screen"
                    }
                    <HomeScreen>{}
                }
                more_view = <StackNavigationView> {
                    header = {
                        content = {
                            title_container = {
                                title = {
                                    text: "More Sccccreeen"
                                }
                            }
                        }
                    }
                    <TextInput> {
                        align: {y: 0.7}
                        margin: {left: 5., top: 6.}
                        width: 100, height: 30,
                        text: "more screenzzzaasnn"
                    }
                    <MoreScreen>{}
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] last_value: i32,
    #[rust] vo: bool,
    #[rust] counter: usize,
    #[rust]
    navigation_destinations: HashMap<StackViewAction, LiveId>,
    #[rust] string_recv: ToUIReceiver<String>,
    #[rust] input_recv: Option<std::sync::mpsc::Sender<String>>,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::shared::styles::live_design(cx);
        crate::shared::helpers::live_design(cx);
        crate::shared::header::live_design(cx);
        crate::shared::search_bar::live_design(cx);
        crate::shared::popup_menu::live_design(cx);
        crate::shared::dropdown_menu::live_design(cx);
        crate::shared::stack_navigation::live_design(cx);
        crate::shared::clickable_view::live_design(cx);
        crate::home::home_screen::live_design(cx);
        crate::more::more_screen::live_design(cx);
    }
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.init_navigation_destinations();
        
    }
}
use std::{thread, fs};
use std::sync::mpsc::channel;
impl App{
    async fn _do_network_request(_cx:CxRef, _ui:WidgetRef, _url:&str)->String{
        "".to_string()
    }
    fn init_navigation_destinations(&mut self) {
        self.navigation_destinations = HashMap::new();
        self.navigation_destinations.insert(StackViewAction::ShowHome, live_id!(home_view));
        self.navigation_destinations.insert(StackViewAction::ShowMore, live_id!(more_view));
    }
    fn handle_network_response(&mut self, cx: &mut Cx, event: &Event) {
        for event in event.network_responses() {
            match &event.response {
                NetworkResponse::HttpResponse(res) => {
                    
                    match event.request_id {
                        live_id!(wasm_test)=>{
                            let label = self.ui.label(id!(label1));
                            println!("-----res: {:?}",res);
                            label.set_text_and_redraw(cx,&format!("Counter: {:?}", res.headers));
                            let data = res.get_body();
                            if let Some(d) = data{
                                let d_c = d.clone();
                                label.set_text_and_redraw(cx,&format!("Counter: start"));
                               
                                label.set_text_and_redraw(cx,&format!("Counter: end"));
                                // match read_foo_text(d){
                                //     Ok(s)=>{
                                //         label.set_text_and_redraw(cx,&format!("Counter: {:?}", s));
                                //     },
                                //     Err(e)=>{
                                //         label.set_text_and_redraw(cx,&format!("Counter: {:?}", e.to_string()));
                                //     }
                                // }
                            }else{
                                label.set_text_and_redraw(cx,&format!("Counter: {:?}", res.headers));
                            }
                        }
                        live_id!(model)=>{
                            let data = res.get_body().unwrap();
                            let label = self.ui.label(id!(model_label));
                            label.set_text_and_redraw(cx,"http fetch");

                            let cache_dir: String = cx.os_type().get_cache_dir().unwrap();

                            fs::write(Path::new(&cache_dir).join(Path::new("tinyllama-1.1b-chat-v0.3.Q5_K_M.gguf")), data).unwrap();
                            label.set_text_and_redraw(cx,"after");

                        }
                        _ =>{}
                    }
                }
                NetworkResponse::WebSocketOpen=>{
                    match event.request_id {
                        live_id!(ws)=>{
                            let label: LabelRef = self.ui.label(id!(label1));
                            label.set_text_and_redraw(cx, "open");
                            cx.web_socket_send_binary(live_id!(ws_msg), b"hello".to_vec())
                        }
                        _=>{}
                    }

                }
                NetworkResponse::WebSocketBinary(bin) => {
                    let label = self.ui.label(id!(label1));
                    label.set_text_and_redraw(cx,std::str::from_utf8(bin).unwrap_or("default"));

                }
                NetworkResponse::WebSocketError(e)=>{
                    let label = self.ui.label(id!(label1));
                    label.set_text_and_redraw(cx,e);

                }
                _=>{}
            }
        }
        
    }
}
use std::{
    fs::File,
    io::prelude::*,
    mem,
    ptr,
    ffi::CStr,
    path::Path
};
use std::net::TcpStream;
use std::sync::{Arc,Mutex};
lazy_static!{
    pub static ref ONCE : Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}
impl AppMain for App{
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }
        if let Event::Signal =event {
            while let Ok(s) = self.string_recv.try_recv() {
                let label: LabelRef = self.ui.label(id!(label1));
                label.set_text_and_redraw(cx, &s);
                return
            }
        }
        let actions = self.ui.handle_widget_event(cx, event);
        
        if self.ui.button(id!(button0)).clicked(&actions) {
            let cache_dir: String = cx.os_type().get_cache_dir().unwrap();
            if fetch_if_noexist(MODEL_URL.into(),cache_dir){
                let request = HttpRequest::new(String::from(MODEL_URL), HttpMethod::GET);
                cx.http_request(live_id!(model), request);
                let label: LabelRef = self.ui.label(id!(model_label));
                label.set_text_and_redraw(cx,"fetching");
            }else{
                let label: LabelRef = self.ui.label(id!(model_label));
                label.set_text_and_redraw(cx,"found model");
            }
        }
        if self.ui.button(id!(button1)).clicked(&actions) {
            let mut user_prompt = self.ui.text_input(id!(message_input)).text();
            user_prompt+="\n";
            if let Some(s) = &self.input_recv{
                let label: LabelRef = self.ui.label(id!(label1));
                
                match s.send(user_prompt){
                    Ok(z)=>{
                        label.set_text_and_redraw(cx, "sending");
                    }
                    Err(e)=>{
                        label.set_text_and_redraw(cx, &format!("e{:?}",e.to_string()));
                    }
                }
            }else{
                if let Ok(ref mut k) = ONCE.clone().lock(){
                    let b = k.clone();
                    if !b{
                        let label: LabelRef = self.ui.label(id!(label1));
                        label.set_text_and_redraw(cx, "once");
                      
                        let string_sender_c: ToUISender<String> = self.string_recv.sender().clone();
                        let string_sender_c2: ToUISender<String> = self.string_recv.sender().clone();
                        let (s,r) = std::sync::mpsc::channel();
                        cx.spawn_thread(move||{
                            
                            let rt = runtime::Runtime::new().unwrap();
                            //string_sender_c2.send(String::from("rt"));
                            let s = rt.block_on(zzz(string_sender_c,r));
                            //string_sender_c2.send(s.unwrap_or(String::from("fff")));
                        });
                        
                        self.input_recv = Some(s.clone());
                        match s.send(user_prompt){
                            Ok(z)=>{
                                label.set_text_and_redraw(cx, "sending");
                            }
                            Err(e)=>{
                                label.set_text_and_redraw(cx, &format!("e{:?}",e.to_string()));
                            }
                        }
                        **k = true
                    }else{
                        // let label: LabelRef = self.ui.label(id!(label1));
                        // label.set_text_and_redraw(cx, "done");
                    }
                }
            }
            
        //---
            
            
            // let mut file = File::create(Path::new(&cache_dir).join("foo.txt")).unwrap();
            // file.write_all(b"Hello, world!").unwrap();
            // self.vo=true;
            // let mut wasm_number = add_two(self.last_value).unwrap_or(0);
            
            //     // let mut f = File::open(Path::new(&cache_dir).join("foo.txt")).unwrap();
            //     // let mut data = String::from("");
            //     // f.read_to_string(&mut data).unwrap();
            let mut wasm_test_exists = false;
            let mut model_exists = false;
            // if fetch_if_noexist(String::from(URL), cache_dir.clone()){
            //     let request = HttpRequest::new(String::from(URL), HttpMethod::GET);
            //     cx.http_request(live_id!(wasm_test), request);
            // }else{
            //     wasm_test_exists = true;
            // }
            // if fetch_if_noexist(String::from(MODEL_URL), cache_dir){
            //     let request = HttpRequest::new(String::from(MODEL_URL), HttpMethod::GET);
            //     cx.http_request(live_id!(model), request);
            // }else{
            //     model_exists = true;
            // }
            // if wasm_test_exists && model_exists{
                
            // }
            // self.last_value =wasm_number;
        }
        self.ui.radio_button_set(ids!(
            mobile_modes.tab1,
            mobile_modes.tab2,
            
        ))
        .selected_to_visible(
            cx,
            &self.ui,
            &actions,
            ids!(
                application_pages.tab1_frame,
                application_pages.tab2_frame,
               
            ),
        );
        self.handle_network_response(cx, event);
        let mut navigation = self.ui.stack_navigation(id!(navigation));
        navigation.handle_stack_view_actions(
            cx,
            &actions,
            &self.navigation_destinations
        );
    }
}