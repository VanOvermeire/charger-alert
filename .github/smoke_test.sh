#!/bin/bash

# TODO could maybe immediately afterwards clean up the data inserted?

echo "Running smoke test by doing a POST to /alert"

base_url=$(aws cloudformation describe-stacks --stack-name ChargerStack --query 'Stacks[0].Outputs[0].OutputValue' --output text)
complete_url="${base_url}/v1/alert"

curl --location --request POST "${complete_url}" \
--fail \
--header 'Content-Type: application/json' \
--data-raw '{
    "ne_lat": 50,
    "ne_lon": 4.4,
    "sw_lat": 49.8,
    "sw_lon": 4.39,
    "email": "test@test.com"
}'
