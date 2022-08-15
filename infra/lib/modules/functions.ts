import {Construct} from "constructs";
import {Code, Function, Runtime} from "aws-cdk-lib/aws-lambda";
import {Duration} from "aws-cdk-lib";
import {REGION} from "./constants";
import {Table} from "aws-cdk-lib/aws-dynamodb";

export const createFunctions = (scope: Construct) => (table: Table) => {
    const alertAdder = new Function(scope, 'AlertAdder', {
        code: Code.fromAsset('../add_alert_build.zip'),
        runtime: Runtime.PROVIDED_AL2,
        handler: 'some.handler',
        timeout: Duration.seconds(3),
        memorySize: 1024,
        environment: {
            REGION,
            TABLE: table.tableName,
        },
    });
    table.grantWriteData(alertAdder);

    const chargeChecker = new Function(scope, 'ChargeChecker', {
        code: Code.fromAsset('../check_charger_build.zip'),
        runtime: Runtime.PROVIDED_AL2,
        handler: 'some.handler',
        timeout: Duration.seconds(60),
        memorySize: 1024,
        environment: {
            REGION,
            TABLE: table.tableName,
        },
    });
    table.grantReadWriteData(chargeChecker);

    return {
        alertAdder,
        chargeChecker,
    }
};
