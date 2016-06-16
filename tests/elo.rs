extern crate skill_rating;

use skill_rating::*;

#[test]
fn it_works() {
}


#[test]
fn elo_match() {
    let john_elo = 1600;
    let james_elo = 1700;

    let (john_elo, james_elo) = elo::game(john_elo, james_elo, 0.5, 32, 32);

    println!("{:?}, {:?}",john_elo, james_elo);


    assert!(true);
}

#[test]
fn elo_series() {
    let john_elo = 1600;
    let tourney_results = [(1200, 1_f32), (1500, 0.5), (1800, 1_f32), (1400, 0_f32)];

    let john_elo = elo::series(john_elo, &tourney_results, 32);

    println!("{:?}",john_elo);


    assert!(true);
}

#[test]
fn elo_icc() {
    let john_elo = 1600;
    let james_elo = 1700;

    let (john_elo, james_elo) = elo::systems::game_icc(john_elo, james_elo, elo::DRAW);

    println!("{:?}, {:?}",john_elo, james_elo);


    assert!(true);
}


