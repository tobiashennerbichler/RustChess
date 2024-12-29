#[cfg(test)]
mod tests {
    use crate::*;
    use crate::parser::notation_parser::{ParsedNotation};
    use crate::piece::piece::{Piece, PieceTypes, Position, Color};
    use crate::game::game::Game;

    #[test]
    fn test_pawn_advances_white() {
        let mut game = Game::new_from_fen("K7/8/8/8/8/8/3P4/k7 w").unwrap();
        let (player, enemy) = game.get_ref_players(Color::White);
        assert_eq!(player.get_pieces().len(), 2);
        assert_eq!(enemy.get_pieces().len(), 1);

        // Check if legal moves are correct
        let white_pawn = player.get_piece(1).to_owned();
        let legal_positions = white_pawn.get_legal_positions(&mut game);
        assert_eq!(legal_positions.len(), 2);
        assert!(legal_positions.contains(&Position::try_from("d3").unwrap()));
        assert!(legal_positions.contains(&Position::try_from("d4").unwrap()));

        // Actually execute move and check board state
        let notation = ParsedNotation::Full(white_pawn.get_position(), Position::try_from("d3").unwrap(), PieceTypes::Pawn);
        game.validate_and_execute_move(notation).unwrap();
        let fen = game.export_fen();
        assert_eq!(fen, "K7/8/8/8/8/3P4/8/k7 w");
        
        game = Game::new_from_fen("K7/8/8/8/8/8/3P4/k7 w").unwrap();
        let (player, enemy) = game.get_ref_players(Color::White);
        let white_pawn = player.get_piece(1).to_owned();
        let notation = ParsedNotation::Full(white_pawn.get_position(), Position::try_from("d4").unwrap(), PieceTypes::Pawn);
        game.validate_and_execute_move(notation).unwrap();
        let fen = game.export_fen();
        assert_eq!(fen, "K7/8/8/8/3P4/8/8/k7 w");
    }

    #[test]
    fn test_pawn_takes_white() {
        let mut game = Game::new_from_fen("K7/8/8/8/8/2ppp3/3P4/k7 w").unwrap();
        let (player, enemy) = game.get_ref_players(Color::White);
        assert_eq!(player.get_pieces().len(), 2);
        assert_eq!(enemy.get_pieces().len(), 4);

        // Check if moves are legal
        let white_pawn = player.get_piece(1).to_owned();
        let legal_positions = white_pawn.get_legal_positions(&mut game);
        assert_eq!(legal_positions.len(), 2);
        assert!(legal_positions.contains(&Position::try_from("c3").unwrap()));
        assert!(legal_positions.contains(&Position::try_from("e3").unwrap()));

        // Actually execute move and check board state
        let notation = ParsedNotation::Full(white_pawn.get_position(), Position::try_from("c3").unwrap(), PieceTypes::Pawn);
        game.validate_and_execute_move(notation).unwrap();
        let fen = game.export_fen();
        assert_eq!(fen, "K7/8/8/8/8/2Ppp3/8/k7 w");
        
        game = Game::new_from_fen("K7/8/8/8/8/2ppp3/3P4/k7 w").unwrap();
        let (player, enemy) = game.get_ref_players(Color::White);
        let white_pawn = player.get_piece(1).to_owned();
        let notation = ParsedNotation::Full(white_pawn.get_position(), Position::try_from("e3").unwrap(), PieceTypes::Pawn);
        game.validate_and_execute_move(notation).unwrap();
        let fen = game.export_fen();
        assert_eq!(fen, "K7/8/8/8/8/2ppP3/8/k7 w");
    }
    
    fn test_pawn_bounds_white() {

    }
}