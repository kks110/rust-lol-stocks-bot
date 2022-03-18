use std::error::Error;
use lol_stocks_core::database:: {
    connection::establish_connection,
    portfolios::load_users_portfolio,
    teams::load_team_by_id,
    users::load_user,
    users::load_user_by_discord_id
};
use lol_stocks_core::models::team::Team;

pub enum PlayerIdentification {
    PlayerId(u64),
    PlayerName(String)
}

pub struct PlayersHoldings {
    pub holdings: Vec<Holding>,
    pub user: String,
    pub balance: i32,
    pub total_value: i32
}

pub struct Holding {
    pub team: Team,
    pub amount: i32,
    pub value: i32
}

pub fn list_holdings_for_player(user_id: PlayerIdentification) -> Result<PlayersHoldings, Box<dyn Error>> {
    let conn = establish_connection();

    let user = match user_id {
        PlayerIdentification::PlayerId(id) => {
            load_user_by_discord_id(&conn, &id)?
        },
        PlayerIdentification::PlayerName(name) => {
            load_user(&conn, &name)?
        }
    };

    let portfolio = load_users_portfolio(&conn, &user)?;

    let mut holdings: Vec<Holding> = Vec::new();
    for item in portfolio {
        let team = load_team_by_id(&conn, &item.team_id)?;
        let value = team.elo * item.amount;
        holdings.push(Holding { team, value, amount: item.amount })
    }
    holdings.sort_by(|a, b| b.value.cmp(&a.value));

    let mut player_holdings = PlayersHoldings{
        holdings: vec![],
        user: user.name.to_string(),
        balance: user.balance,
        total_value: 0
    };

    let mut value = 0;
    value += user.balance;

    for holding in holdings {
        value += holding.value;
        player_holdings.holdings.push(holding);
    }

    player_holdings.total_value = value;

    Ok(player_holdings)
}