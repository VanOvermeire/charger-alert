# Charger Alert

Demo application

// TODO:
//  Lambda 1: call to rest endpoint with location triangle -> save in dynamodb
//  Lambda 2: on cron, check if anything in dynamo - if so, do call to url and see if charger is available -> if so, send email (and delete request in dynamo)
//  Components: lambda, cron, dynamo
//  optionally pass in a name or id of a charging point

## Features

## Status

![Github Actions Status](https://github.com/VanOvermeire/charger-alert/actions/workflows/github-deploy.yml/badge.svg)
