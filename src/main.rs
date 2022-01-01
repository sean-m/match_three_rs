
mod board;


fn main() {
    
    let mut board = board::GameBoard::new(4, 2);
    let token = board::Piece::new("G","","");
    board.place_token(token,1,2);

    let token2 = board::Piece::new("G","","");
    board.place_token(token2,2,1);

    let token2 = board::Piece::new("R","","");
    board.place_token(token2,2,3);

    println!("Board {} by {} is {} big.", board.width, board.height, board.get_size());
    println!("{:#?}", board);
}
