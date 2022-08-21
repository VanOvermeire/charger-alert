import {Construct} from "constructs";
import {Code, Function, Runtime} from "aws-cdk-lib/aws-lambda";
import {Duration} from "aws-cdk-lib";
import {Table} from "aws-cdk-lib/aws-dynamodb";

const handler = 'some.handler';

export const createFunctions = (scope: Construct) => (table: Table, region: string, source_email: string) => {
    const alertAdder = new Function(scope, 'AlertAdder', {
        handler,
        code: Code.fromAsset('../add_alert.zip'),
        runtime: Runtime.PROVIDED_AL2,
        timeout: Duration.seconds(3),
        memorySize: 1024,
        environment: {
            REGION: region,
            TABLE: table.tableName,
        },
    });
    table.grantWriteData(alertAdder);

    const chargeChecker = new Function(scope, 'ChargeChecker', {
        handler,
        code: Code.fromAsset('../check_charger.zip'),
        runtime: Runtime.PROVIDED_AL2,
        timeout: Duration.seconds(60),
        memorySize: 1024,
        environment: {
            REGION: region,
            TABLE: table.tableName,
            EMAIL: source_email,
        },
    });
    table.grantReadWriteData(chargeChecker);

    return {
        alertAdder,
        chargeChecker,
    }
};
