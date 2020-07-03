use crate::{NetlistObject,json::*,Logger};
pub struct Netlist
{
    pub nodes: Vec<NetlistObject>,
    pub module_names: Vec<String>,
    pub modules: Vec<JsonValue>,
}

impl Netlist
{
    pub fn new() -> Netlist
    {
        Netlist
        {
           nodes: Vec::new(),
           module_names: Vec::new(),
           modules: Vec::new(),
        }
    }
    pub fn parse_json(&mut self,logger: &mut Logger, strptr: &str)
    {
        let mut json: JsonValue = json::parse(strptr).unwrap();
        self.nodes.truncate(0);
        self.modules.truncate(0);
        logger.log("Parsing JSON");
        for (k,p) in json.entries()
        {
            if(k.to_string().eq("creator"))
            {
                logger.log(&("Creator: ".to_owned()+&p.to_string()));
            }
            else if(k.to_string().eq("modules"))
            {
                for (k2,p2) in p.entries()
                {
                    logger.log(&("Found a module: ".to_owned()+&k2.to_string()));
                    self.module_names.push(k2.to_string());
                    self.modules.push(p2.to_owned());
                }
            }
            //logger.log(&k.to_string());
        }
        if(self.modules.len()==0)
        {
            logger.log(&"Could not find any modules =(");
        }
        else if(self.modules.len()==1)
        {
            self.unpack_module(logger,0);
        }
        else
        {
           logger.log(&"Multiple modules detected, select one to unpack with the 'Find Object' button"); 
        }
    }
    //Find an object based on name. Returns None if unsuccessful
    pub fn find_object(&mut self,logger: &mut Logger, id: &str) -> Option<&NetlistObject>
    {
        for n in &mut self.nodes
        {
            if(n.name.eq(id))
            {
                logger.log(&("Found ".to_owned()+id));
                return Some(n);
            }
        }
        logger.log(&("Could not find a node called ".to_owned()+id));
        None
    }
    //Returns a one off reference to a node's index in the array based on coordinates. Returns 0 if no node is found
    pub fn get_clicked_node(&mut self,logger: &mut Logger, xpos: f64,ypos: f64) -> usize
    {
        let mut i: usize = 1;
        for n in &self.nodes
        {
            if(xpos>n.xpos&&ypos>n.ypos&&xpos<(n.xpos+n.width)&&ypos<(n.ypos+n.height))
            {
                return i;
            }
            i = i + 1;
        }
        return 0;
    }
    //Finds a module based on name
    pub fn find_and_unpack_module(&mut self, logger: &mut Logger,id: &str)
    {
        if(self.modules.len()==0)
        {
            logger.log("You need to load a netlist first");
            return;
        }
        let mut spot: usize = 0;
        for n in &self.module_names
        {
            if(n.eq(id))
            {
                    self.unpack_module(logger,spot);
                    return;
            }
            spot = spot + 1;
        }
        logger.log(&("Could not find a module called ".to_owned()+id));
    }
    pub fn unpack_module(&mut self, logger: &mut Logger, index: usize)
    {
        let mod_obj: JsonValue = self.modules.get(index).unwrap().to_owned();
        for (k,p) in mod_obj.entries()
        {
            if(k.to_string().eq("cells"))
            {
                for (k2,p2) in p.entries()
                {
                    let new_node: NetlistObject = NetlistObject::new(k2.to_owned());
                    
                    self.nodes.push(new_node);
                }
            }
            else if(k.to_string().eq("ports"))
            {
                for (k2,p2) in p.entries()
                {
                    let new_node: NetlistObject = NetlistObject::new(k2.to_owned());
                    
                    self.nodes.push(new_node);
                }
            }
        }
        logger.log(&("Unpacked module ".to_owned()+self.module_names.get(index).unwrap()));
        self.place_nodes(logger);
    }
    //Place all unpacked nodes down
    //Currently places them in a square without connecting wires, as a force based
    //placement algorithm like that used by elkJS needs to be found or written
    pub fn place_nodes(&mut self,logger: &mut Logger)
    {
        //Grid placement algorithm
        let X_START: f64 = 100.0;
        let Y_START: f64 = 100.0;
        let SPACING: f64 = 150.0;
        let across_count: i32 = (self.nodes.len() as f64).sqrt() as i32 + 1;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for node in &mut self.nodes
        {
            node.xpos = X_START + (x as f64 * SPACING);
            node.ypos = Y_START + (y as f64 * SPACING);
            x = x + 1;
            if(x==across_count)
            {
                x = 0;
                y = y + 1;
            }
        }
    }
}