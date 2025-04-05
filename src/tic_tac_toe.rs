
struct Matrix {
    value: Vec<Vec<char>>,
    total_completed: u8
}

impl Matrix {
    fn new() -> Matrix {
        Matrix { value: vec![vec![' ',' ',' '],vec![' ',' ',' '],vec![' ',' ',' ']], total_completed:0 }
    }
    fn set_at(&mut self,index:usize, character: char) -> bool{
        if index > 8  {
            false
        }else{
            let (row,column) = Matrix::get_row_column(index) ;
            if self.value[row][column] != ' '{
                false
            }else{
                self.value[row][column] = character ;
                self.total_completed += 1;
                true
            }
        }
    }

    fn get_row_column(index:usize) -> (usize,usize) {
            let row ;
            let column;
           if index <= 2 {
            row = 0 ;
            column = index ;
           }else if index >= 3 && index <= 5 {
            row = 1 ;
            column = index - 3 ;
           }else{
            row = 2 ;
            column = index - 6 ; 
           }
           (row,column)
    }

    /* 
    
    [0, 1, 2]
    [3, 4, 5]
    [6, 7, 8]
    */

    fn winner_check(&self,turn: char, index: usize) -> char {
        // checking the winner is the main part left
        let (row,column) = Matrix::get_row_column(index) ;
        // let's do row and column check first
        let (first,second) =  if column == 1 { (column-1, column+1) }else{ if column == 2 { (column-1 , column-2)}else {
            (column+1, column+2)
        } }  ;
        let (first_row,second_row) =  if row == 1 { (row-1, row+1) }else{ if row == 2 { (row-1 , row-2)}else {
            (row+1, row+2)
        } }  ;
        if self.value[row][first] == turn && self.value[row][second] == turn {
            turn
        }else{
            //check for row
            if self.value[first_row][column] == turn && self.value[second_row][column] == turn{
                turn
            }
            else{
                // now check for the diagonals
                if self.value[0][0] == turn && self.value[1][1] == turn && self.value[2][2] == turn {
                    turn
                }else if self.value[0][2] == turn && self.value[1][1] == turn && self.value[2][0] == turn {
                    turn
                }else{
                    // if total  were completed return 'D' else ' '
                    if self.total_completed == 9 { 'D' }else{ ' ' }
                }
            }
        }

    }

    fn print_matrix(&self) {

        let mut i = 0 ;
        println!("-------------") ;
        while i < 3 {
            let first = if self.value[i][0] == ' ' { (3*i).to_string() }else{ self.value[i][0].to_string() } ;
            let second = if self.value[i][1] == ' '{ ((3*i)+1).to_string() }else{ self.value[i][1].to_string()} ;
            let third = if self.value[i][2] == ' ' {((3*i)+2).to_string()}else { self.value[i][2].to_string()} ;
            println!("| {} | {} | {} |",first,second,third ) ;
            i += 1;
        }
        println!("-------------") ;
    }

}




use std::io::stdin ;
pub fn tic_tac_toe() {
    println!("Let's Play the game") ;
         let mut x_turn = true ;
         let mut tic_tac_toe = Matrix::new() ;
         loop {
            tic_tac_toe.print_matrix();
            println!("{} please choose the box", if x_turn{'x'}else{'o'}) ;
             let mut input = String::new() ;
             stdin().read_line(&mut input).expect("Error During input process") ;
             input = input.trim().to_string();
             let value_check = input.to_string().into_bytes()[0] ;
             if  value_check >= 48 && value_check <= 57 {
                println!("input was {}", input);
                let index: usize = input.parse().unwrap();
                let out_put = tic_tac_toe.set_at(index, if x_turn { 'x' }else{ 'o' }) ;
                if out_put {
                   let winner = tic_tac_toe.winner_check(if x_turn {'x'}else{'o'},index) ;
                     if  winner != ' ' {
                        if winner == 'D' {
                            println!("it's a Draw!! 🚀") ;
                        }else{
                            println!("winner was ⚡ {} 🔥", winner) ;
                            tic_tac_toe.print_matrix();
                        }

                        break;
                     }
                     x_turn = !x_turn ;
                }else{
                    println!("Make Sure choose an index that's not completed and in the range") ;
                }

             }else{
                println!("Make sure enter only the the number") ;
             }
            
         }
}
