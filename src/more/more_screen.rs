use makepad_widgets::*;
use crate::shared::clickable_view::*;
use crate::shared::stack_view_action::StackViewAction;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    MoreScreen = <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true,

        <TextInput> {
            align: {y: 0.7}
            margin: {left: 5., top: 6.}
            width: 100, height: 30,
            text: "more screen11nnn"
        }
    }
    
}
