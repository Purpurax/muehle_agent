pub fn list_moves<F>(positions: [Token; 24], token_type: Token, phase: Phase) -> impl Iterator<Item=Move> {
    let empty_positions = positions.iter().enumerate()
        .filter(|(_, token)| **token == Token::None);
        
    if phase == Phase::Set {
        return empty_positions
            .map(|(end_position, _)| (None, end_position))
            // .for_each(|position| callback(position));
        // for (end_position, end_token) in positions.iter().enumerate() {
        //     if *end_token != Token::None {
        //         continue;
        //     }

        //     callback((None, end_position))
        // }
    } else {
        let number_of_token = get_number_of_token(positions, token_type);

        return positions.iter().enumerate()
            .filter(|(_, token)| **token == token_type)
            .flat_map(|(start_position, _)| {
                empty_positions.filter(|(end_position, end_token)| is_move_valid(start_position, *end_position, **end_token, number_of_token))
                    .map(|(end_position, _)| (Some(start_position), *end_position))
            });
            
        // for (start_position, &token) in beatable_token_iter {
        //     if token_type != token {
        //         continue;
        //     }
    
        //     for (end_position, end_token) in positions.iter().enumerate() {
        //         if is_move_valid(start_position, end_position, *end_token, number_of_token_type) {
        //             callback((Some(start_position), end_position))
        //         }
        //     }
        // }
    }
}