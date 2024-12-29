#[cfg(test)]
mod tests {
    use crate::*;
    use crate::parser::notation_parser::{ParsedNotation};
    use crate::piece::piece::{Piece, PieceTypes, Position};

    struct State(Vec<Player>, Board);

    fn setup(fen: &str) -> State {
        let mut players = Vec::new();
        let mut board = Board::new();
        load_state_from_fen(fen, &mut players, &mut board).expect("Should only provide valid FEN notation");
        State(players, board)
    }

    #[test]
    fn test_pawn_advances_white() {
        let State(mut players, mut board) = setup("K7/8/8/8/8/8/3P4/k7 w");
        assert_eq!(players[0].get_pieces().len(), 2);
        assert_eq!(players[1].get_pieces().len(), 1);

        let (player, enemy) = get_mut_player_enemy(&mut players, 0);

        let white_pawn = player.get_piece(1).to_owned();
        let legal_positions = white_pawn.get_legal_positions(player, enemy, &mut board);
        assert_eq!(legal_positions.len(), 2);
        assert!(legal_positions.contains(&Position::try_from("d3").unwrap()));
        assert!(legal_positions.contains(&Position::try_from("d4").unwrap()));

        let notation = ParsedNotation::Full(white_pawn.get_position(), Position::try_from("d3").unwrap(), PieceTypes::Pawn);
        board.validate_and_execute_move(player, enemy, notation).unwrap();
        // TODO: implement exportFEN and test it with that
    }

    #[test]
    fn test_pawn_takes_white() {
        let State(mut players, mut board) = setup("K7/8/8/8/8/2ppp3/3P4/k7 w");
        assert_eq!(players[0].get_pieces().len(), 2);
        assert_eq!(players[1].get_pieces().len(), 4);

        let (player, enemy) = get_mut_player_enemy(&mut players, 0);

        let white_pawn = player.get_piece(1).to_owned();
        let legal_positions = white_pawn.get_legal_positions(player, enemy, &mut board);
        println!("{legal_positions:?}");
        assert_eq!(legal_positions.len(), 2);
        assert!(legal_positions.contains(&Position::try_from("c3").unwrap()));
        assert!(legal_positions.contains(&Position::try_from("e3").unwrap()));
    }
    
    fn test_pawn_bounds_white() {

    }
}