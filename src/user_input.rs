use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct UserInput
{
    pub mouse_down: bool,
    pub pending_click: bool,
    pub pending_move: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    last_mouse_x: i32,
    last_mouse_y: i32,
    pub wheel_delta: f64,
}
impl UserInput
{
    pub fn new() -> UserInput
    {
        UserInput
        {
           mouse_down: false,
           pending_click: false,
           pending_move: false,
           mouse_x: 0,
           mouse_y: 0,
           last_mouse_x: 0,
           last_mouse_y: 0,
           wheel_delta: 0.0,
        }
    }
    pub fn on_mouse_down(&mut self, event: web_sys::MouseEvent)
    {
        self.mouse_down = true;
        self.pending_click = true;
    }
    pub fn on_mouse_up(&mut self, event: web_sys::MouseEvent)
    {
        self.mouse_down = false;
    }
    pub fn on_scroll(&mut self,event: web_sys::WheelEvent)
    {
        self.wheel_delta = event.delta_y();
    }
    pub fn on_mouse_move(&mut self,event: web_sys::MouseEvent)
    {
        self.pending_move = true;
        self.mouse_x = event.offset_x();
        self.mouse_y = event.offset_y();
    }
    pub fn on_mouse_leave(&mut self,event: web_sys::MouseEvent)
    {
        self.mouse_down = false;
    }
    pub fn get_move_x(&mut self) -> i32
    {
        return self.mouse_x - self.last_mouse_x;
    }
    pub fn get_move_y(&mut self) -> i32
    {
        return self.mouse_y - self.last_mouse_y;
    }
    pub fn dequeue_move(&mut self)
    {
        self.pending_move = false;
        self.last_mouse_x = self.mouse_x;
        self.last_mouse_y = self.mouse_y;
    }
}