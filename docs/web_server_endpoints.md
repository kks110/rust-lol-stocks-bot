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
    {
      "name": "TSM",
      "league": "LCS"
    },
    {
      "name": "FNC",
      "league": "LEC"
    }
  ]
}
```

##### POST /padlock
A way to lock and unlock the database without needed to use discord.

```JSON
{
	"unlock": true
}
```