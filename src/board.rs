


#[derive(Clone,Copy,Eq, PartialEq,Debug)]
pub enum Cellprop{
    Free=0,
    Wall=1,
    Mouse=2
}

#[derive(Clone,Copy)]
pub struct Cell{
    prop:Cellprop
}

#[derive(Clone)]
pub struct Board{
    pub dimcol:i32,
    pub dimlin:i32,
    pub matrix:[[Cell;15];15],
    pub mouse_pos:(i32,i32),
}
pub fn any_difference(a:&Board, b:&Board) -> bool {
    for i in 1..12 {
        for j in 1..12{
            if a.matrix[i as usize][j as usize].prop!=b.matrix[i as usize][j as usize].prop {
                return true;
            }
        }
    }
    return false;
}
pub fn gameboard_state(a:&Board) -> char{
    // 0 - no win; 1 - player 1 won; 2 - player 2 won
    let sor=a.mouse_pos;
    if sor.0*sor.1 ==0 || sor.0>11 || sor.1>11 {
        return 'M';
    }

    for diri in 1..6  {
        if a.test_move_mouse(diri) {
            return '0';
        }
    }

    return 'W';
}
pub fn wall_position(a:&Board) -> Vec<(i32,i32)> {
    let mut wallpos=Vec::new();

    for i in 1..12 {
        for j in 1..12{
            if a.matrix[i as usize][j as usize].prop==Cellprop::Wall {
                let pereche=(i,j);
                wallpos.push(pereche);
            }
        }
    }
    wallpos

}
impl Board{

    pub fn new() -> Self{
        let free_cell:Cell= Cell{prop:Cellprop::Free};
        let mut mat:[[Cell;15];15]=[[free_cell;15];15];
        mat[6][6].prop=Cellprop::Mouse;
        
        Board{dimcol:11, dimlin:11, matrix:mat, mouse_pos:(6,6)}
    }


    pub fn make_wall(&mut self, pos:(i32,i32)) -> bool {
        println!("OKKKKK, deci pe pozitia {}, {}: e exact {:?}",pos.0,pos.1,(*self).matrix[pos.0 as usize][pos.1 as usize].prop);
        if (*self).matrix[pos.0 as usize][pos.1 as usize].prop as isize == 0 {
            (*self).matrix[pos.0 as usize][pos.1 as usize].prop=Cellprop::Wall;
            return true;
        }
        return false;
    } 

    pub fn test_move_mouse(&self, dir:i32) -> bool { //Tested: works well
        let sor=(*self).mouse_pos;
        let mut newsor:(i32,i32)=sor;
        let mut dx:Vec<i32>=Vec::new();
        let mut dy:Vec<i32>=Vec::new();

        if sor.0 % 2 == 0 {
            dx.extend([0,-1,-1,0,1,1,0]);
            dy.extend([0,0,1,1,1,0,-1]);
            
        }
        else {
            dx.extend([0,-1,-1,0,1,1,0]);
            dy.extend([0,-1,0,1,0,-1,-1]);
        }
        newsor.0=newsor.0+dx[dir as usize];
        newsor.1=newsor.1+dy[dir as usize];
        
        if (*self).matrix[newsor.0 as usize][newsor.1 as usize].prop as isize == 0 {
            return true;
        }
        return false;
        
    }
    pub fn move_mouse(&mut self, dir:i32) -> bool { //Tested: works well
        let sor=(*self).mouse_pos;
        let mut newsor:(i32,i32)=sor;
        let mut dx:Vec<i32>=Vec::new();
        let mut dy:Vec<i32>=Vec::new();

        if sor.0 % 2 == 0 {
            dx.extend([0,-1,-1,0,1,1,0]);
            dy.extend([0,0,1,1,1,0,-1]);
            
        }
        else {
            dx.extend([0,-1,-1,0,1,1,0]);
            dy.extend([0,-1,0,1,0,-1,-1]);
        }
        newsor.0=newsor.0+dx[dir as usize];
        newsor.1=newsor.1+dy[dir as usize];
        
        if (*self).matrix[newsor.0 as usize][newsor.1 as usize].prop as isize == 0 {
            (*self).matrix[sor.0 as usize][sor.1 as usize].prop=Cellprop::Free;
            (*self).matrix[newsor.0 as usize][newsor.1 as usize].prop=Cellprop::Mouse;
            (*self).mouse_pos=newsor;
            return true;
        }
        return false;
        
    }
    pub fn set_mouse(&mut self, newsor:(i32,i32)) {
        self.matrix[self.mouse_pos.0 as usize][self.mouse_pos.1 as usize].prop=Cellprop::Free;
        (*self).matrix[newsor.0 as usize][newsor.1 as usize].prop=Cellprop::Mouse;
        self.mouse_pos=newsor;

    }
    pub fn print_for_debug(&self) {

        let print_vec:Vec<char>=vec!['_','W','M'];
        println!("_________________________\n");
        for i in 1..12 {
            if i%2==0 {
                print!(" ");
            }
            for j in 1..12{
                print!("{} ", print_vec[(*self).matrix[i][j].prop as usize]);
            }
            print!("\n");
        }
        println!("\n_________________________");
    }

    pub fn translate_to_moves(&mut self,who_moves:char) -> String{
        let mut translated=String::from("");
        //{M/W - who moves}{M/W/0 - game state}{x,y-mouse position}{x,y-wall_pos1}{x,y-wall_pos2}...{x,y-wall_pos_k}
        translated.push('{');
        translated.push(who_moves);
        translated.push('}');
        
        let st=gameboard_state(&self);
        translated.push('{');
        translated.push(st);
        translated.push('}');


        translated.push('{');
        translated.push_str(self.mouse_pos.0.to_string().as_str());
        translated.push(',');
        translated.push_str(self.mouse_pos.1.to_string().as_str());
        translated.push('}');
        
        for it in wall_position(&self) {
            translated.push('{');
            translated.push_str(it.0.to_string().as_str());
            translated.push(',');
            translated.push_str(it.1.to_string().as_str());
            translated.push('}');
        }
        translated
    }
    
    pub fn translate_to_board(&mut self, newboardmove:String) {
        let mut ind=0;
        let mut nb=Board::new();
        let mut wx=0;
        let mut wy;
        for wrd in newboardmove.split(['{','}',',']) {
            if ind==0 {
                //who moved
            }
            else if ind==1{
                //who won
            }
            else {
                wy=wrd.parse().unwrap();
                if ind%2==1 && ind/2>1 {
                    nb.make_wall((wx,wy));
                }
                else if ind%2==1 {
                    //mouse
                    nb.set_mouse((wx,wy));
                }
                wx=wrd.parse().unwrap();
            }
            ind=ind+1;
        }
        for i in 1..12{
            for j in 1..12{
                self.matrix[i][j]=nb.matrix[i][j];
            }
        }
        self.mouse_pos=nb.mouse_pos;
    }
}