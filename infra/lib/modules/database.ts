import {AttributeType, Table} from "aws-cdk-lib/aws-dynamodb";
import {Construct} from "constructs";
import {CfnOutput} from "aws-cdk-lib";

export const createDatabase = (scope: Construct) => {
    const table = new Table(scope, 'ChargerAlertTable', {
        partitionKey: {name: 'id', type: AttributeType.STRING},
    });

    new CfnOutput(scope, 'databaseName', {
        value: table.tableName,
        description: 'The name of the application database',
    });

    return table;
};
