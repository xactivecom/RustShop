## Run
Run command-line and see help:
    % cargo install --path . && cmd_app --help
    % cargo run arg1 arg2


# Management Operations

## Collect data
Collect data from the API to local files.
Syntax:
    --source "nhl" | "sportradar"
    --entity "team-image" | "team-season" | "roster" | "player"
    [ --season YYY1YYY2 | YYY1YYY2:YYY3YYY4 ]
    [ --team abbrev ]

Examples:
    % collect --source nhl --entity team-image
    % ALTERNATE: collect --source nhl --entity team-image --team MTL

    % collect --source nhl --entity team-season
    % ALTERNATE: collect --source nhl --entity team-season --season 20242025
    % ALTERNATE: collect --source nhl --entity team-season --season 20202021:20232024

    % collect --source nhl --entity roster --season 20242025

    % collect --source nhl --entity player --season 20242025

## Load data
Load data from local files into the database.
Syntax:
    --source "nhl" | "sourceradar"
    --entity "league" | "season" | "team" | "roster" | "player"
    [ --season YYY1YYY2 | YYY1YYY2:YYY3YYY4 ]
    [ --team abbrev ]

Examples:
    % load --source nhl --entity league

    % load --source nhl --entity season --league NHL

    % load --source nhl --entity team --league NHL
    % ALTERNATE: load --source nhl --entity team --league NHL --season 20242025
    
    % load --source nhl --entity roster --league NHL
    % ALTERNATE: load --source nhl --entity roster --league NHL --season 20242025
    % ALTERNATE: load --source nhl --entity roster --league NHL --season 20202021:20232024
