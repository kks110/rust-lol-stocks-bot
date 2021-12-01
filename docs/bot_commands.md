### Discord Server Commands

##### register
```
!!register
```
Register to play. This will create an account and give you a starting balance.

##### view_market
```
!!view_market <league_name?>
```
Shows the market price of all the teams with no args sent. Otherwise, will just show team in the specified league.

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
!!view_portfolio <user?>
```
Shows a users portfolio and how much its worth. If no user is specified it will show your own.

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

##### market_cap
```
!!market_cap
```

Show how many of what shares are owned.

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