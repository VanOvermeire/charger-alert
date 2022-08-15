import {Construct} from "constructs";
import {Code, Function, Runtime} from "aws-cdk-lib/aws-lambda";
import {Duration} from "aws-cdk-lib";
import {REGION} from "./constants";
import {Table} from "aws-cdk-lib/aws-dynamodb";

export const createFunctions = (scope: Construct) => (table: Table) => {
    const chargerAlert = new Function(scope, 'ChargerAlert', {
        code: Code.fromAsset('../charger-alert.zip'),
        runtime: Runtime.PROVIDED_AL2,
        handler: 'some.handler',
        timeout: Duration.seconds(30),
        memorySize: 1024,
        environment: {
            REGION,
            TABLE: table.tableName,
        },
    });

    table.grantWriteData(chargerAlert);

    return {
        chargerAlert,
    }
};
