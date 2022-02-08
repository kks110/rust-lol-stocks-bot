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

struct Holding {
    pub team: Team,
    pub amount: i32,
    pub value: i32
}


pub fn make_portfolio_view(user_id: PlayerIdentification) -> Result<String, Box<dyn Error>> {
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

    let mut response: String = String::from("");

    let mut value = 0;

    let user_balance = format!("User: {}, Balance: {}\n", user.name, user.balance);
    value += user.balance;
    response.push_str(&user_balance);

    for holding in holdings {
        value += holding.value;
        let portfolio_line = format!("Team: {}, Amount: {}, Value: {}\n", holding.team.name, holding.amount, holding.value);
        response.push_str(&portfolio_line);
    }

    let total_value = format!("Total Portfolio Value: {}", value);
    response.push_str(&total_value);
    Ok(response)
}
