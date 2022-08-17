#!/bin/bash

echo "Running smoke test by doing a POST to /alert"

base_url=$(aws cloudformation describe-stacks --stack-name ChargerStack --query 'Stacks[0].Outputs[0].OutputValue' --output text)
complete_url="${base_url}/v1/alert"

curl --location --request POST "${complete_url}" \
--fail \
--header 'Content-Type: application/json' \
--data-raw '{
    "ne_lat": 9,
    "ne_lon": 6.1,
    "sw_lat": 22,
    "sw_lon": 55.1
}'
