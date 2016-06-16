pub mod systems;

pub type EloRating = i32;
pub const WIN: f32 = 1_f32;
pub const DRAW: f32 = 0.5;
pub const LOSS: f32 = 0_f32;

fn rating_change(k: u32, score: f32, exp_score: f32) -> EloRating {
    (k as f32 * (score - exp_score)) as i32
}

pub fn expected_score(r_a: EloRating, r_b: EloRating) -> f32 {
    1_f32 / (1_f32 + 10_f32.powf(((r_b - r_a) as f32) / 400_f32))
}


pub fn game(r_a: EloRating, r_b: EloRating, s_a: f32, k_a: u32, k_b: u32) -> (EloRating, EloRating) {
    let s_b = 1_f32 - s_a;

    let e_a = expected_score(r_a, r_b);
    let e_b = 1_f32 - e_a;


    let new_a = r_a + rating_change(k_a, s_a, e_a);
    let new_b = r_b + rating_change(k_b, s_b, e_b);


    (new_a, new_b)
}

pub fn series(r_a: EloRating, games: &[(EloRating, f32)], k_factor: u32) -> EloRating {
    let mut score = 0_f32;
    let mut exp_score = 0_f32;

    for game in games {
        score += game.1;
        exp_score = expected_score(r_a, game.0);
    }

    r_a + (k_factor as f32 * (score - exp_score)) as i32
}

