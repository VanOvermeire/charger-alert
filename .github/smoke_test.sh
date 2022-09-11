#!/bin/bash

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
    "email": "test@test.com",
    "charger_id": 1
}'

echo "Removing test items from table"
# make sure our new item will be found - three seconds should be plenty
sleep 3

# TODO get name from outputs
for item in $(aws dynamodb scan --table-name ChargerStack-ChargerAlertTableED5D11D8-W37RU5U8VTS5 --filter-expression "email = :name" --expression-attribute-values '{":name":{"S":"test@test.com"}}' --projection-expression id --query 'Items[]' | jq -rc '.[]'); do
  aws dynamodb delete-item --table-name ChargerStack-ChargerAlertTableED5D11D8-W37RU5U8VTS5 --key $item
done
