use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Logger
{
    has_console: bool,
    console: web_sys::HtmlTextAreaElement,
    output: String,
    pending_print: bool,
}
impl Logger
{
    pub fn new(console_id: &str) -> Logger
    {
        let console: web_sys::HtmlTextAreaElement;
        let has_console = false;
        //Add in " is N/A" check here
        let pre_console = web_sys::window().unwrap().document().unwrap().get_element_by_id(console_id).unwrap();
        console = pre_console
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .map_err(|_| ())
        .unwrap();
        //has_console = true;
        Logger
        {
            has_console,
            console,
            output: "Console output here".to_string(),
            pending_print: false,
        }
    }
    //This maybe be optimizable
    pub fn log(&mut self, msg: &str)
    {
        self.output.push_str("\n");
        self.output.push_str(msg);
        self.pending_print = true;
    }
    pub fn attempt_print(&mut self)
    {
        if(self.pending_print)
        {
            self.pending_print = false;
            self.console.set_value(&self.output);
            self.console.set_scroll_top(self.console.scroll_height());
        }
    }
}