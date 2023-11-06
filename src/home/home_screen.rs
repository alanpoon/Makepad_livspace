use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    HomeScreen = <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true,

        <TextInput> {
            align: {y: 0.7}
            margin: {left: 5., top: 6.}
            width: 100, height: 30,
            text: "home screen bb"
        }
    }
    
}