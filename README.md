# Charger Alert

Demo application

## TODO

Lambda 1: call to rest endpoint with location triangle -> save in dynamodb
Lambda 2: on cron, check if anything in dynamo - if so, do call to url and see if charger is available -> if so, send email (and delete request in dynamo)

Optionally pass in a name or id of a charging point

## Features

## Status

![Github Actions Status](https://github.com/VanOvermeire/charger-alert/actions/workflows/github-deploy.yml/badge.svg)

## Remarks

- Once passed the checker (which was annoying), code worked immediately (ignoring the "I did not give the Lambda permission to invoke Dynamo" part)
- Similar for macros, just work (and useful)
- Compiling takes a while, as is well-known
- Switching to workspaces was easy locally, but caused some strange issues in github actions (with the openssl dependency)