extern crate skill_rating;

use skill_rating::*;




#[test]
fn elo_match_win() {
    let john_elo = 1600;
    let james_elo = 1700;

    let (john_elo, james_elo) = elo::game(john_elo, james_elo, elo::WIN, 32, 32);


    assert_eq!(john_elo, 1620);
    assert_eq!(james_elo, 1680);
}

#[test]
fn elo_match_draw() {
    let john_elo = 1600;
    let james_elo = 1700;

    let (john_elo, james_elo) = elo::game(john_elo, james_elo, elo::DRAW, 32, 32);


    assert_eq!(john_elo, 1604);
    assert_eq!(james_elo, 1696);
}

#[test]
fn elo_match_loss() {
    let john_elo = 1600;
    let james_elo = 1700;

    let (john_elo, james_elo) = elo::game(john_elo, james_elo, elo::LOSS, 32, 32);


    assert_eq!(john_elo, 1589);
    assert_eq!(james_elo, 1711);
}


#[test]
fn elo_series() {
    let john_elo = 1600;
    let tourney_results = [(1200, 1_f32), (1500, 0.5), (1800, 1_f32), (1400, 0_f32)];

    let john_elo = elo::series(john_elo, &tourney_results, 32);


    assert_eq!(john_elo, 1655);
}

#[test]
fn elo_icc() {
    let john_elo = 1600;
    let james_elo = 1700;

    let (john_elo, james_elo) = elo::systems::game_icc(john_elo, james_elo, elo::DRAW);


    assert_eq!(john_elo, 1604);
    assert_eq!(james_elo, 1696);
}


