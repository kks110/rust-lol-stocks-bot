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
`!!view_market` > Shows the market price of all the teams.
`!!buy <team> <amount to buy>` > Buys 'Shares' in a team and add to your portfolio.
`!!sell <team> <amount to buy>` > Sells 'Shares' in a team, removes from your portfolio and add to your balance.
`!!view_portfolio` > Shows your portfolio and how much its worth.
`!!elo_history_for <team>` > Shows the historic prices of the team.
`!!portfolio_performance` > Shows your portfolios historic values.

The ability to buy and sell will be locked over the weekend to make sure that the games have been updated before people try to buy and sell teams.
`!!db_lock`  > Can be used by admins to lock and unlock the market.
";


#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}