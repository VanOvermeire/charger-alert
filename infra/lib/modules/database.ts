import {AttributeType, Table} from "aws-cdk-lib/aws-dynamodb";
import {Construct} from "constructs";

export const createDatabase = (scope: Construct) => {
    return new Table(scope, 'ChargerAlertTable', {
        partitionKey: {name: 'id', type: AttributeType.STRING},
    });
};
