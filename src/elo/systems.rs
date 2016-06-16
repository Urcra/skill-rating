use super::*;

pub fn fide_k(rating: EloRating, games: u32) -> u32 {
    if games < 30 {
        40
    } else if rating < 2400 {
        20
    } else {
        10
    }
}

pub fn uscf_k(rating: EloRating) -> u32 {
    if rating < 2100 {
        32
    } else if rating < 2400 {
        24
    } else {
        16
    }
}


pub fn icc_k() -> u32 {
    32
}


pub fn game_fide(r_a: EloRating, g_a: u32, r_b: EloRating, g_b: u32, s_a: f32) -> (EloRating, EloRating) {
    let k_a = fide_k(r_a, g_a);
    let k_b = fide_k(r_b, g_b);

    game(r_a, r_b, s_a, k_a, k_b)
}

pub fn game_uscf(r_a: EloRating, r_b: EloRating, s_a: f32) -> (EloRating, EloRating) {
    let k_a = uscf_k(r_a);
    let k_b = uscf_k(r_b);

    game(r_a, r_b, s_a, k_a, k_b)
}


pub fn game_icc(r_a: EloRating, r_b: EloRating, s_a: f32) -> (EloRating, EloRating) {
    let k_a = icc_k();
    let k_b = icc_k();

    game(r_a, r_b, s_a, k_a, k_b)
}