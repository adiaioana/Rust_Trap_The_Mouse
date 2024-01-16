pub fn str_to_int(a:&str) -> i32{
    let mut nr=0;
    let mut maybenotpossible=true;
    for ch in a.chars() {
        if ch.is_numeric(){
            maybenotpossible=false;
            nr=nr*10+(ch as u8 -'0' as u8) as i32;
        }
    }
    if maybenotpossible {
        return -1;
    }
    return nr;
}
pub struct BasicMessages{
    youwon:String,
    youlost:String,
    placeawall:String,
    movemouse:String
}

impl BasicMessages{
    pub fn new() ->Self{
        BasicMessages{youwon:String::from("You won (against all walls), congrats!"),youlost:String::from("You lost(in a trap the mouse game?)"),
    placeawall:String::from("Place a wall(2 integers):"), movemouse:String::from("Move mouse in one of the 6 directions[1-6]>")}
    }
    pub fn lostmess(&self) ->String{
        self.youlost.clone()
    }
    pub fn wonmess(&self) -> String{
        self.youwon.clone()
    }
    pub fn movemousemess(&self) ->String{
        self.movemouse.clone()
    }
    pub fn placewallmess(&self) -> String {
        self.placeawall.clone()
    }
}