use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{netlist::*,netlist_object::*,user_input::*};
const SCALE_RATE: f64 = 1.2;
pub struct Display
{
	canvas: web_sys::HtmlCanvasElement,
	context: web_sys::CanvasRenderingContext2d,
    skin: web_sys::HtmlImageElement,
    pan_x: f64,
    pan_y: f64,
    scale_factor: f64,
}

impl Display
{
    pub fn new(canvas_id: &str,skin_id: &str) -> Display
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let pre_canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas = pre_canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
        let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
        let pre_skin = document.get_element_by_id(skin_id).unwrap();
        let skin = pre_skin
        .dyn_into::<web_sys::HtmlImageElement>()
        .map_err(|_| ())
        .unwrap();
        Display
        {
           canvas,
           context,
           skin,
           pan_x: 0.0,
           pan_y: 0.0,
           scale_factor: 1.0,
        }
    }
    pub fn scale(&mut self,delta: f64)
    {
        let w: f64 = self.canvas.width() as f64;
        let h: f64 = self.canvas.height() as f64;
        if(delta<0.0)//Zoom in
        {
            self.pan_x = self.pan_x - (w * 0.5 * (1.0-(1.0/SCALE_RATE)) / self.scale_factor);
            self.pan_y = self.pan_y - (h * 0.5 * (1.0-(1.0/SCALE_RATE)) / self.scale_factor);
            self.scale_factor = self.scale_factor * SCALE_RATE;
        }
        else//Zoom out
        {
            self.pan_x = self.pan_x + (w * 0.5 * (1.0-(1.0/SCALE_RATE)) / self.scale_factor);
            self.pan_y = self.pan_y + (h * 0.5 * (1.0-(1.0/SCALE_RATE)) / self.scale_factor);
            self.scale_factor = self.scale_factor / SCALE_RATE;
        }
    }
    pub fn pan(&mut self,delta_x: i32,delta_y: i32)
    {
        self.pan_x = self.pan_x + ((delta_x as f64) / self.scale_factor);
        self.pan_y = self.pan_y + ((delta_y as f64) / self.scale_factor);
    }
    pub fn pan_to_object(&mut self, obj: &NetlistObject)
    {
        let w: f64 = self.canvas.width() as f64;
        let h: f64 = self.canvas.height() as f64;
        self.pan_x = -(obj.xpos - (w * 0.5 / self.scale_factor));
        self.pan_y = -(obj.ypos - (h * 0.5 / self.scale_factor));
    }
    pub fn redraw(&mut self,net: &Netlist, user_input: &UserInput)
    {
       self.canvas.set_width(self.canvas.width());//Wipes the canvas
       //self.context.draw_image_with_html_image_element(&self.skin,400.0,400.0);
       for node in &net.nodes
       {
           self.draw_single(node);
       }
       self.context.stroke();
    }
    //Draw one object
    pub fn draw_single(&mut self, obj: &NetlistObject)
    {
        let xpos: f64 = ((obj.xpos + self.pan_x)) * self.scale_factor;
        let ypos: f64 = ((obj.ypos + self.pan_y)) * self.scale_factor;
        let obj_width: f64 = obj.width * self.scale_factor;
        let obj_height: f64 = obj.height * self.scale_factor;
        //Currently draws a placeholder rectangle, as the skin file is not parsed yet
        self.context.rect(xpos,ypos,obj_width,obj_height);
        self.context.fill_text(&obj.name,xpos,(ypos-10.0));
    }
    //Convert X or Y in screen pixels to a location in the schematic
    pub fn get_true_x(&mut self,x: i32) -> f64
    {
        return ((x as f64) / self.scale_factor) - self.pan_x;
    }
    pub fn get_true_y(&mut self,y: i32) -> f64
    {
        return ((y as f64) / self.scale_factor) - self.pan_y;
    }
}