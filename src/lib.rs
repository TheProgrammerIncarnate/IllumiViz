use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use json;

mod display;
mod user_input;
mod netlist;
mod netlist_object;
mod logger;
use crate::{display::*,user_input::*,netlist::*,netlist_object::*,logger::*};

const MAX_FPS: u32 = 30;

#[wasm_bindgen]
pub struct Top
{
    last_tick_time: u32,
    logger: Logger,
	display: Display,
    user_input: UserInput,
    netlist: Netlist,
    num_dragged: usize,
    drag_offset_x: f64,
    drag_offset_y: f64,
}

#[wasm_bindgen]
impl Top
{
    pub fn new(canvas_id: &str,skin_id: &str,console_id: &str) -> Top
    {
        Top
        {
            last_tick_time: 0,
            logger: Logger::new(console_id),
            display: Display::new(canvas_id,skin_id),
            user_input: UserInput::new(),
            netlist: Netlist::new(),
            num_dragged: 0,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,
        }
    }
    //These functions pass events to the user input handler
    pub fn on_mouse_down(&mut self,event: web_sys::MouseEvent)
    {
        self.user_input.on_mouse_down(event);
    }
    pub fn on_mouse_up(&mut self,event: web_sys::MouseEvent)
    {
        self.user_input.on_mouse_up(event);
    }
    pub fn on_scroll(&mut self,event: web_sys::WheelEvent)
    {
        self.user_input.on_scroll(event);
    }
    pub fn on_mouse_move(&mut self,event: web_sys::MouseEvent)
    {
        self.user_input.on_mouse_move(event);
    }
    pub fn on_mouse_leave(&mut self,event: web_sys::MouseEvent)
    {
        self.user_input.on_mouse_leave(event);
    }
    //Called by a requestAnimationFrame() loop in host page
    pub fn update(&mut self,timestamp: u32)
    {
        //Throttle to the max FPS so as to not bog down the CPU
        if(timestamp-self.last_tick_time<(1000/MAX_FPS)){return;};
        self.last_tick_time = timestamp;
        //web_sys::console::log_1(&format!("{}",timestamp).into());
        if(self.user_input.pending_click)
        {
            self.user_input.pending_click = false;
            let truex: f64 = self.display.get_true_x(self.user_input.mouse_x);
            let truey: f64 = self.display.get_true_y(self.user_input.mouse_y);
            //Set num_dragged to the position of the node we clicked (0 = none was selected)
            self.num_dragged = self.netlist.get_clicked_node(&mut self.logger,truex,truey);
            if(self.num_dragged!=0)
            {
                self.drag_offset_x = self.netlist.nodes[self.num_dragged-1].xpos - truex;
                self.drag_offset_y = self.netlist.nodes[self.num_dragged-1].ypos - truey;
            }
        }
        if(self.user_input.pending_move)
        {
            if(self.user_input.mouse_down)
            {
                if(self.num_dragged!=0)
                {
                    //Pan the selected node
                    self.netlist.nodes[self.num_dragged - 1].xpos = self.display.get_true_x(self.user_input.mouse_x) + self.drag_offset_x;
                    self.netlist.nodes[self.num_dragged - 1].ypos = self.display.get_true_y(self.user_input.mouse_y) + self.drag_offset_y;
                }
                else
                {
                    //Pan the screen
                    self.display.pan(self.user_input.get_move_x(),self.user_input.get_move_y());
                }
            }
            self.user_input.dequeue_move();
        }
        if(self.user_input.wheel_delta!=0.0)
        {
            self.display.scale(self.user_input.wheel_delta);
            self.user_input.wheel_delta = 0.0;
        }
        self.display.redraw(&self.netlist,&self.user_input);
        self.logger.attempt_print();
    }
    pub fn load_netlist(&mut self, json: &str)
    {
        self.netlist.parse_json(&mut self.logger,json);
    }
    pub fn find_object(&mut self,id: &str)
    {
        //Find and pan to an object. If our netlist is packed, then attempt to unpack a module
        if(self.netlist.nodes.len()==0)
        {
            self.netlist.find_and_unpack_module(&mut self.logger,id);
        }
        else
        {
            let maybe_object: Option<&NetlistObject> = self.netlist.find_object(&mut self.logger,id);
            match maybe_object
            {
                Some(obj) => self.display.pan_to_object(obj),
                None => (),
            }
        }
    }
}