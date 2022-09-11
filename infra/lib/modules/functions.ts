import {Construct} from "constructs";
import {Code, Function, Runtime} from "aws-cdk-lib/aws-lambda";
import {Duration} from "aws-cdk-lib";
import {Table} from "aws-cdk-lib/aws-dynamodb";
import {Effect, PolicyStatement} from "aws-cdk-lib/aws-iam";

const handler = 'some.handler';

const createRustFunction = (scope: Construct) => (name: string, zip: string, duration = 3, environment: Record<string, string> = {}) =>
    new Function(scope, name, {
        handler,
        code: Code.fromAsset(zip),
        runtime: Runtime.PROVIDED_AL2,
        timeout: Duration.seconds(duration),
        memorySize: 1024,
        environment,
    })

export const createFunctions = (scope: Construct) => (table: Table, region: string, source_email: string) => {
    const createRustFunctionForScope = createRustFunction(scope);

    const getChargers = createRustFunctionForScope('GetChargers', '../get_chargers.zip');

    const alertAdder = createRustFunctionForScope('AlertAdder', '../add_alert.zip', 3, {
        REGION: region,
        TABLE: table.tableName,
    });
    table.grantWriteData(alertAdder);

    const chargeChecker = createRustFunctionForScope('ChargeChecker', '../check_charger.zip', 60, {
        REGION: region,
        TABLE: table.tableName,
        EMAIL: source_email,
    });
    chargeChecker.addToRolePolicy(new PolicyStatement({
        actions: ['ses:SendEmail', 'SES:SendRawEmail'],
        resources: ['*'],
        effect: Effect.ALLOW,
    }));
    table.grantReadWriteData(chargeChecker);

    return {
        getChargers,
        alertAdder,
        chargeChecker,
    }
};
