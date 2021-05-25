use crate::models::teams::Team;

pub fn calculate_elo(winning_team: &Team, loosing_team: &Team) ->  (i64, i64) {
    let winner_elo = winning_team.elo as f64;
    let loser_elo = loosing_team.elo as f64;

    let winner_expected = expected_score(winner_elo, loser_elo);
    let loser_expected = expected_score(loser_elo, winner_elo);

    let new_winner_elo =  elo_calculation(winner_elo, winner_expected, true);
    let new_looser_elo = elo_calculation(loser_elo, loser_expected, false);
    return (new_winner_elo, new_looser_elo)
}

fn expected_score(player_rating: f64, opponent_rating: f64) -> f64 {
    1.0/(1.0 + (10.0_f64.powf((opponent_rating - player_rating)/400.0)))
}

fn elo_calculation(player_elo: f64, expected_elo: f64, win: bool) -> i64 {
    let mut i = 0.0;
    if win {
        i = 1.0;
    }
    (player_elo + 16.0 * (i - expected_elo)) as i64
}
