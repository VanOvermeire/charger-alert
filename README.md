# Charger Alert

Demo application

## Status

![Github Actions Status](https://github.com/VanOvermeire/charger-alert/actions/workflows/github-deploy.yml/badge.svg)

## Phase 1

Lambda 1: save a requested location to database
Lambda 2: on cron, for every location in the db see if a charger is available and if so email the user

Optionally pass in a name or id of a charging point

## Phase 2

Lambda 1: for a request location, show possible charging stations
Lambda 2: save a specific charging station
Lambda 3: on cron, see if a charger is available in the given station and if so email the user

## Features

- Automatic testing, building, deploying and smoke testing with GitHub actions. Whole flow takes about 10 minutes
- Infra as code with CDK, including tests
- Rust for the backend code

## Remarks

- Once passed the checker (which was annoying), code worked immediately (ignoring the "I did not give the Lambda permission to invoke Dynamo" part)
- Similar for macros, just work (and useful)
- Compiling takes a while, as is well-known
- Switching to workspaces was easy locally, but caused some strange issues in github actions (with the openssl dependency)
- Await infects large parts of the code base - FP ideas can help limit this
- Also see remarks in the code itself
