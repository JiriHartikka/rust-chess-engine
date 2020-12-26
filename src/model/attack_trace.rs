use super::game_state::Position;

pub fn attack_trace_for_rook() -> Vec<Vec<Vec<Position>>> {
    let mut trace = Vec::new();

    for rank in 1..9 {
        for file in 1..9 {
            let starting_pos = Position::new(file, rank);

            let file_trace1 = trace_with_delta(starting_pos, 1, 0);
            let file_trace2 = trace_with_delta(starting_pos, -1, 0);
            let rank_trace1 = trace_with_delta(starting_pos, 0, 1);
            let rank_trace2 = trace_with_delta(starting_pos, 0, -1);

            trace.push(vec![file_trace1, file_trace2, rank_trace1, rank_trace2]);
        }
    }

    trace
}

pub fn attack_trace_for_bishop() -> Vec<Vec<Vec<Position>>> {
    let mut trace = Vec::new();

    for rank in 1..9 {
        for file in 1..9 {
            let starting_pos = Position::new(file, rank);

            let diagonal_trace1 = trace_with_delta(starting_pos, 1, 1);
            let diagonal_trace2 = trace_with_delta(starting_pos, -1, -1);
            let diagonal_trace3 = trace_with_delta(starting_pos, -1, 1);
            let diagonal_trace4 = trace_with_delta(starting_pos, 1, -1);

            trace.push(vec![diagonal_trace1, diagonal_trace2, diagonal_trace3, diagonal_trace4])
        }
    }

    trace
}

pub fn attack_trace_for_queen() -> Vec<Vec<Vec<Position>>> {
    let mut queen_trace = attack_trace_for_rook();
    queen_trace.append(&mut attack_trace_for_bishop());
    queen_trace
}

pub fn attack_trace_for_knight() -> Vec<Vec<Vec<Position>>> {
    let mut trace = Vec::new();

    for rank in 1..9 {
        for file in 1..9 {
            let starting_pos = Position::new(file, rank);
            let mut trace_at_pos = Vec::new();

            let candidates: Vec<Position> = vec![
                starting_pos.delta(1, 2), 
                starting_pos.delta(1, -2),
                starting_pos.delta(-1, 2),
                starting_pos.delta(-1, -2),
                starting_pos.delta(2, 1),
                starting_pos.delta(2, -1),
                starting_pos.delta(-2, 1),
                starting_pos.delta(-2, -1),
            ].into_iter().filter_map(|x| x).collect();

            for x in candidates {
                trace_at_pos.push(vec![x]);
            }
            trace.push(trace_at_pos);
        }
    }

    trace
}


fn trace_with_delta(starting_pos: Position, delta_file: i8, delta_rank: i8) -> Vec<Position> {
    let mut trace = Vec::new();
    let mut current_position = starting_pos.delta(delta_file, delta_rank);

    while let Some(position) = current_position {
        trace.push(position);
        current_position = position.delta(delta_file, delta_rank);
    }

    trace
}