
mod board;


fn main() {
    
    let board = board::GameBoard::new(8, 8);

    println!("Board {} by {} is {} big.", board.width, board.height, board.get_size());
}
