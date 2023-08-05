#[cfg(test)]
use crate::model::attack_trace;
#[cfg(test)]
use crate::model::game_state::Position;
#[cfg(test)]
use std::fmt::Debug;

#[test]
fn test_knight_trace() {
    let knight_trace = attack_trace::attack_trace_for_knight();

    let corner_trace = &knight_trace[usize::from(Position::new(1, 1).to_numeric())];
    let expected = vec![vec![Position::new(2, 3)], vec![Position::new(3, 2)]];
    assert_eq!(&expected, corner_trace);

    let trace_from_h4 = &knight_trace[usize::from(Position::new(8, 4).to_numeric())];
    let expected_h4 = vec![
        vec![Position::new(7, 6)],
        vec![Position::new(7, 2)],
        vec![Position::new(6, 5)],
        vec![Position::new(6, 3)],
    ];
    assert_eq!(&expected_h4, trace_from_h4);
}

#[test]
fn test_rook_trace() {
    let mut rook_trace = attack_trace::attack_trace_for_rook();

    let trace_from_f7 = &mut rook_trace[usize::from(Position::new(6, 7).to_numeric())];

    let mut expected = vec![
        vec![Position::new(6, 8)],
        vec![
            Position::new(6, 6),
            Position::new(6, 5),
            Position::new(6, 4),
            Position::new(6, 3),
            Position::new(6, 2),
            Position::new(6, 1),
        ],
        vec![
            Position::new(5, 7),
            Position::new(4, 7),
            Position::new(3, 7),
            Position::new(2, 7),
            Position::new(1, 7),
        ],
        vec![Position::new(7, 7), Position::new(8, 7)],
    ];
    assert_eq_any_order(&mut expected, trace_from_f7);
}

#[test]
fn test_bishop_trace() {
    let mut bishop_trace = attack_trace::attack_trace_for_bishop();
    let trace_from_a2 = &mut bishop_trace[usize::from(Position::new(1, 2).to_numeric())];

    let mut expected = vec![
        vec![],
        vec![
            Position::new(2, 3),
            Position::new(3, 4),
            Position::new(4, 5),
            Position::new(5, 6),
            Position::new(6, 7),
            Position::new(7, 8),
        ],
        vec![],
        vec![Position::new(2, 1)],
    ];

    assert_eq_any_order(&mut expected, trace_from_a2);
}

#[cfg(test)]
fn assert_eq_any_order<T: Ord + Debug>(a: &mut Vec<T>, b: &mut Vec<T>) {
    a.sort();
    b.sort();

    assert_eq!(a, b);
}
