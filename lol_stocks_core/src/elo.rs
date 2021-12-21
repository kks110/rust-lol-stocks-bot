pub fn calculate_elo(winning_team_elo: i32, loosing_team_elo: i32) ->  (i32, i32) {
    let winner_elo = winning_team_elo as f64;
    let loser_elo = loosing_team_elo as f64;

    let winner_expected = expected_score(winner_elo, loser_elo);
    let loser_expected = expected_score(loser_elo, winner_elo);

    let new_winner_elo =  elo_calculation(winner_elo, winner_expected, true);
    let new_looser_elo = elo_calculation(loser_elo, loser_expected, false);
    (new_winner_elo, new_looser_elo)
}

fn transform_elo(elo: &f64) -> f64 {
    10.0_f64.powf(elo / 400.0)
}

fn expected_score(player_rating: f64, opponent_rating: f64) -> f64 {
    let player_transformed = transform_elo(&player_rating);
    let opponent_transformed = transform_elo(&opponent_rating);

    player_transformed / (player_transformed + opponent_transformed)
}

fn elo_calculation(player_elo: f64, expected_elo: f64, win: bool) -> i32 {
    let mut i = 0.0;
    if win {
        i = 1.0;
    }
    (player_elo + 16.0 * (i - expected_elo)) as i32
}
