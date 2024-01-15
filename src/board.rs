
#[derive(Clone,Copy,Eq, PartialEq,Debug)]
pub enum CellProp{
    Free=0,
    Wall=1,
    Mouse=2
}

#[derive(Clone,Copy)]
pub struct cell{
    Prop:CellProp
}

pub struct Board{
    pub dimcol:i32,
    pub dimlin:i32,
    pub matrix:[[cell;15];15],
    pub mouse_pos:(i32,i32)
}

impl Board{

    pub fn new() -> Self{
        let mut freecell:cell= cell{Prop:CellProp::Free};
        let mut mat:[[cell;15];15]=[[freecell;15];15];
        mat[6][6].Prop=CellProp::Mouse;
        
        Board{dimcol:11, dimlin:11, matrix:mat, mouse_pos:(6,6)}
    }

    pub fn state(&self, turn:i32) -> i32{
        // 0 - no win; 1 - player 1 won; 2 - player 2 won
        let sor=(*self).mouse_pos;
        if sor.0*sor.1 ==0 || sor.0>11 || sor.1>11 {
            return turn;
        }
        return 0;
    }

    pub fn make_wall(&mut self, pos:(i32,i32)) -> bool {
        println!("OKKKKK, deci pe pozitia {}, {}: e exact {:?}",pos.0,pos.1,(*self).matrix[pos.0 as usize][pos.1 as usize].Prop);
        if (*self).matrix[pos.0 as usize][pos.1 as usize].Prop as isize == 0 {
            (*self).matrix[pos.0 as usize][pos.1 as usize].Prop=CellProp::Wall;
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
        
        if (*self).matrix[newsor.0 as usize][newsor.1 as usize].Prop as isize == 0 {
            (*self).matrix[sor.0 as usize][sor.1 as usize].Prop=CellProp::Free;
            (*self).matrix[newsor.0 as usize][newsor.1 as usize].Prop=CellProp::Mouse;
            (*self).mouse_pos=newsor;
            return true;
        }
        return false;
        
    }
    pub fn print_for_debug(&self) {

        let print_vec:Vec<char>=vec!['_','W','M'];
        println!("_________________________\n");
        for i in 1..12 {
            if i%2==0 {
                print!(" ");
            }
            for j in 1..12{
                print!("{} ", print_vec[(*self).matrix[i][j].Prop as usize]);
            }
            print!("\n");
        }
        println!("\n_________________________");
    }
}