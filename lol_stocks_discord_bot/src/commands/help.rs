use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use crate::helpers::messages;

const HELP_MESSAGE_TITLE: &str = "
Here is a quick rundown of the things you can do:
(All teams names are their abbreviations).

";

const HELP_MESSAGE_COMMANDS: [(&str, &str, bool); 20] = [
    ("`!!register <alias?>`", "Register to play. This will create an account and give you a starting balance. Can also add an alias", false),
    ("`!!update_alias <alias>`", "Add a user alias to yourself", false),
    ("`!!market <league?>`", "Shows the market price of all the teams with no args sent. Otherwise, will just show team in the specified league.", false),
    ("`!!buy <amount to buy> <team>`", "Buys 'Shares' in a team and add to your portfolio.", false),
    ("`!!buy_all <team>`", "Buys as many 'Shares' in a team as you can afford.", false),
    ("`!!sell <amount to sell> <team>`", "Sells 'Shares' in a team, removes from your portfolio and add to your balance.", false),
    ("`!!sell_all <team?>`", "Sells all 'Shares' in a specified team, if no team is specified, will sell your whole portfolio.", false),
    ("`!!portfolio <user?>`", "Shows a users portfolio and how much its worth. If no user is specified it will show your own.", false),
    ("`!!team_history <team>`", "Shows the historic prices of the team.", false),
    ("`!!team_history_graph <team>`", "Generated a graph based on elo and history of a team.", false),
    ("`!!portfolio_history <user?>`", "Shows users portfolios historic values, yours by default.", false),
    ("`!!portfolio_history_graph <user?>`", "Generated a graph based on users portfolio value and history, yours by default.", false),
    ("`!!leaderboard`", "Shows all users and their portfolio values.", false),
    ("`!!leaderboard_graph`", "Generated a graph based on all users portfolio value and history.", false),
    ("`!!market_cap`", "Show how many of what shares are owned.", false),
    ("`!!schedule`", "Show links to the schedule sites.", false),
    ("`!!weekly_report`", "Shows the +/- for the last week.", false),
    ("`!!suggest <league?>`", "Suggests you a random team from one or both markets", false),
    ("`!!suggest_affordable <league?>`", "Suggests you a random team from one or both markets that you have the money to buy", false),
    ("`!!market_lock`", "Can be used by admins to lock and unlock the market.", false),
];

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    messages::send_message(
        ctx,
        msg,
        "Welcome to the Stock Market Summoner!",
        Some(HELP_MESSAGE_TITLE),
        Some(Vec::from(HELP_MESSAGE_COMMANDS))
    ).await?;
    Ok(())
}
