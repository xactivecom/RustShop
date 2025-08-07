# Management Operations

Demonstrate using the `clap` crate to parse command-lines needed for the Sports Portal project.

## Collect data
Collect data from the provider API to local files.
Syntax:
    --source "nhl" | "sportradar"
    --entity "teamimage" | "teamseason" | "roster" | "player"
    [ --season YYY1YYY2 | YYY1YYY2:YYY3YYY4 ]
    [ --team abbrev ]

Examples:
    % collect --source nhl --entity teamimage
    % ALTERNATE: collect --source nhl --entity teamimage --team MTL

    % collect --source nhl --entity team-season
    % ALTERNATE: collect --source nhl --entity teamseason --season 20242025
    % ALTERNATE: collect --source nhl --entity teamseason --season 20202021:20232024

    % collect --source nhl --entity roster --season 20242025

    % collect --source nhl --entity player --season 20242025

## Load data
Load data from local files into the portal database.
Syntax:
    --source "nhl" | "sourceradar"
    --entity "league" | "season" | "team" | "roster" | "player"
    [ --season YYY1YYY2 | YYY1YYY2:YYY3YYY4 ]
    [ --team abbrev ]

Examples:
    % load --source nhl --entity league

    % load --source nhl --entity season --league nhl

    % load --source nhl --entity team --league nhl
    % ALTERNATE: load --source nhl --entity team --league nhl --season 20242025
    
    % load --source nhl  --league nhl --entity roster
    % ALTERNATE: load --source nhl  --league nhl --entity roster --season 20242025
    % ALTERNATE: load --source nhl  --league nhl --entity roster --league NHL --season 20202021:20232024

## Demonstration
Examples:

    % cargo run collect --source nhl --entity teamseason --season 20242025
    % cargo run load --source nhl  --league nhl --entity roster --season 20242025
