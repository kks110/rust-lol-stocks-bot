use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

const HELP_MESSAGE: &str = "
Welcome to the Stock Market Summoner!

Here is a quick rundown of the things you can do:

All teams names are their abbreviations and in capitals.

`!!register` > Register to play. This will create an account and give you a starting balance.
`!!view_market <league?>` > Shows the market price of all the teams with no args sent. Otherwise, will just show team in the specified league.
`!!buy <amount to buy> <team>` > Buys 'Shares' in a team and add to your portfolio.
`!!buy_all <team>` > Buys as many 'Shares' in a team as you can afford.
`!!sell <amount to sell> <team>` > Sells 'Shares' in a team, removes from your portfolio and add to your balance.
`!!sell_all <team?>` > Sells all 'Shares' in a specified team, if no team is specified, will sell your whole portfolio.
`!!view_portfolio <user?>` > Shows a users portfolio and how much its worth. If no user is specified it will show your own.
`!!elo_history_for <team>` > Shows the historic prices of the team.
`!!team_graph <team>` > Generated a graph based on elo and history of a team.
`!!portfolio_performance <user?>` > Shows users portfolios historic values, yours by default.
`!!portfolio_graph <user?>` > Generated a graph based on users portfolio value and history, yours by default.
`!!leaderboard` > Shows all users and their portfolio values.
`!!leaderboard_graph` > Generated a graph based on all users portfolio value and history.
`!!market_cap` > Show how many of what shares are owned.
`!!schedule` > Show links to the schedule sites.
`!!weekly_report` > Shows the +/- for the last week.

The ability to buy and sell will be locked over the weekend to make sure that the games have been updated before people try to buy and sell teams.
`!!db_lock`  > Can be used by admins to lock and unlock the market.
";


#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;
    Ok(())
}
