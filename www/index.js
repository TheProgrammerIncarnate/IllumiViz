import { Top } from "illumi-viz"
const top = Top.new("main_canvas","skin","console");
let canvas = document.getElementById("main_canvas");
canvas.addEventListener("mousedown",event => {top.on_mouse_down(event)});
canvas.addEventListener("mouseup",event => {top.on_mouse_up(event)});
canvas.addEventListener("wheel",event => {top.on_scroll(event)});
canvas.addEventListener("mousemove",event => {top.on_mouse_move(event)});
canvas.addEventListener("mouseleave",event => {top.on_mouse_leave(event)});
let load_button = document.getElementById("loadnet_button");
load_button.addEventListener("click", event => {top.load_netlist(document.getElementById("netlist_entry").value)});
let find_button = document.getElementById("find_button");
find_button.addEventListener("click", event => {top.find_object(document.getElementById("search_entry").value)});
let updateFunction = (timestamp) =>
{
	top.update(timestamp);
	requestAnimationFrame(updateFunction);
};
updateFunction();
