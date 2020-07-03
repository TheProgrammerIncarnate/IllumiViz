pub struct NetlistObject
{
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub xpos: f64,
    pub ypos: f64,
}

impl NetlistObject
{
    pub fn new(name: String) -> NetlistObject
    {
        NetlistObject
        {
           name,
           width: 40.0,
           height: 40.0,
           xpos: 0.0,
           ypos: 0.0,
        }
    }
}