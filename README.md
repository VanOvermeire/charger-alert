# Charger Alert

Demo application (written for a medium blog post) that alerts you when there are available connectors for recharging your car.

## Status

![Github Actions Status](https://github.com/VanOvermeire/charger-alert/actions/workflows/github-deploy.yml/badge.svg)

## Requirements

- an AWS account with credentials added to Github Actions secrets
- SES sandbox if you're sending between your own email addresses. the 'source email' Github secret should be a verified email

## Phase 1

- Lambda 1: save a requested location to database
- Lambda 2: on cron, for every location in the db see if a charger is available and if so email the user

## Phase 2

- Lambda 1: for a request location, show possible charging stations
- Lambda 2: save a specific charging station
- Lambda 3: on cron, see if a charger is available in the given station and if so email the user

## Features

- Automatic testing, building, deploying and smoke testing with GitHub actions. Whole flow takes about 10 minutes
- IaC with CDK, including infra tests
- Rust for the backend code

## Limitations, shortcoming

- config has an optional email address, but it would be even better not to have this present at all in the functions that don't need it
- some inconsistencies in approaches (use of traits, generics, hiding or exposing of info), partly caused by me experimenting with approaches
- error handling incomplete: mapping to our own types, but not doing anything with the underlying errors, which would make debugging issues harder
- some string copying could probably have been avoided (*but* most of it is in tests, which makes it less of an issue)
- repeating the lat/long info in get-chargers and add-alert is not ideal. One way to improve this, would be to save the info we get back from get-chargers in a database - so we know all the relevant info when the add call happens

## Remarks

Also see comments in the code itself.
 
- Once passed the checker (which was annoying), code worked immediately (well, except for me forgetting the right permissions in the IaC part)
- Macros avoid some annoying work and are not very hard to write
- Switching to workspaces was easy locally, but caused some strange issues in GitHub actions (with the openssl dependency)
- Compiling takes a while, as is well-known
- Await infects large parts of the code base, though a functional approach can help *limit* this
