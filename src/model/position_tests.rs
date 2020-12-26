use super::game_state::Position;

#[test]
fn to_bit_mask() {
    let a1_bit_mask = Position::new(1, 1).to_bit_mask();
    assert_eq!(1, a1_bit_mask);

    let b1_bit_mask = Position::new(2, 1).to_bit_mask();
    assert_eq!(1 << 1, b1_bit_mask);

    let a2_bit_mask = Position::new(1, 2).to_bit_mask();
    assert_eq!(1 << 8, a2_bit_mask);

    let d5_bit_mask = Position::new(4, 5).to_bit_mask();
    assert_eq!(1 << (4 * 8  + 3), d5_bit_mask);

    let h8_bit_mask = Position::new(8, 8).to_bit_mask();
    assert_eq!(1 << 63, h8_bit_mask);
}

#[test]
fn numeric_position() {
    let a1 = Position::new(1, 1);
    assert_eq!(0, a1.to_numeric());
    
    let h1 = Position::new(8, 1);
    assert_eq!(7, h1.to_numeric());

    let a2 = Position::new(1, 2);
    assert_eq!(8, a2.to_numeric());

    let d7 = Position::new(4, 7);
    assert_eq!(6 * 8 + 3, d7.to_numeric());
}

#[test]
fn display() {
    let a1 = Position::new(1, 1);
    assert_eq!("A1", format!("{}", a1));
    
    let h1 = Position::new(8, 1);
    assert_eq!("H1", format!("{}", h1));

    let a2 = Position::new(1, 2);
    assert_eq!("A2", format!("{}", a2));

    let d7 = Position::new(4, 7);
    assert_eq!("D7", format!("{}", d7));
}

#[test]
fn file() {
    let a1 = Position::new(1, 1);
    assert_eq!(1, a1.file());
    
    let h1 = Position::new(8, 1);
    assert_eq!(8, h1.file());

    let a2 = Position::new(1, 2);
    assert_eq!(1, a2.file());

    let d7 = Position::new(4, 7);
    assert_eq!(4, d7.file());
}

#[test]
fn rank() {
    let a1 = Position::new(1, 1);
    assert_eq!(1, a1.rank());
    
    let h1 = Position::new(8, 1);
    assert_eq!(1, h1.rank());

    let a2 = Position::new(1, 2);
    assert_eq!(2, a2.rank());

    let d7 = Position::new(4, 7);
    assert_eq!(7, d7.rank());
}

#[test]
fn delta() {
    let a1 = Position::new(1, 1);
    assert_eq!(Option::Some(Position::new(3, 2)), a1.delta(2, 1));
    assert_eq!(Option::None, a1.delta(-2, 1));
    assert_eq!(Option::None, a1.delta(2, -1));
    assert_eq!(Option::Some(Position::new(2, 3)), a1.delta(1, 2));
}