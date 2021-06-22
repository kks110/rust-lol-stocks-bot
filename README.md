# LCS / LEC Stock Market

A discord bot and web server to let you run a League of Legends stock market. You can 'buy' and 'sell' teams based on their elo, log the games each week and see the prices rise and fall.

A guide on how to get this up and runnig on a raspbery pi can be found [here](./docs/pi_setup.md)

### Discord Server Commands

##### register
```
!!register
```
Register to play. This will create an account and give you a starting balance.

##### view_market
```
!!view_market
```
Shows the market price of all the teams.

##### buy
```
!!buy <team> <amount to buy>
```
Buys 'Shares' in a team and add to your portfolio.

##### buy_all
```
!!buy_all <team>
```
Buys as many 'Shares' in a team as you can afford.

##### sell
```
!!sell <team> <amount to buy>
```
Sells 'Shares' in a team, removes from your portfolio and add to your balance.

##### sell_all
```
!!sell_all <team?>
```
Sells all 'Shares' in a specified team, if no team is specified, will sell your whole portfolio.

##### view_portfolio
```
!!view_portfolio
```
Shows your portfolio and how much its worth.

##### portfolio_graph
```
!!portfolio_graph
``` 
Generated a graph based on portfolio value and history.

##### elo_history_for
```
!!elo_history_for <team>
```
Shows the historic prices of the team.

##### team_graph
```
!!team_graph <team>
```
Generated a graph based on elo and history of a team.

##### portfolio_performance
```
!!portfolio_performance
```
Shows your portfolios historic values.

##### leaderboard
```
!!leaderboard
```
Shows all users, and their portfolio values.

##### leaderboard_graph
```
!!leaderboard_graph
```
Generated a graph based on all users portfolio value and history.

##### weekly_reports
```
!!weekly_reports
```
View the current price of all the teams, but also show the increase / decrease from last week.

##### schedule
```
!!schedule
```
Show links to the schedule sites.

##### db_lock
```
!!db_lock
```
The ability to buy and sell will be locked over the weekend to make sure that the games have been updated before people try to buy and sell teams.
Can be used by admins to lock and unlock the market.

##### ping
```
!!ping
```
Returns pong. Easy way to test if the server is running.


### Web Server Endpoints

##### POST /register_matches
Register the weeks matches. Post a json in the below format. It will update the teams ELO's and it will take a snapshot of all the teams elos and the players portfolio prices which is used by the portfolio_performance and elo_history_for commands.

```JSON
{
  "matches": [
    {
      "winner": "Winning Team Name",
      "looser": "Loosing Team Name"
    },
    {
      "winner": "Winning Team Name",
      "looser": "Loosing Team Name"
    }
  ]
}
```

##### POST /register_teams
For registering the teams initially. A ready to go payload with all the LEC and LCS teams can be found in the docs folder called `teams_db_seed.json`

```JSON
{
  "teams": [
    "team_name",
    "team_name",
    "team_name"
  ]
}
```

##### POST /padlock
A way to lock and unlock the database without needed to use discord.

```JSON
{
	"unlock": bool
}
```