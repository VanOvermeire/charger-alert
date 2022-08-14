import {Construct} from "constructs";
import {Code, Function, Runtime} from "aws-cdk-lib/aws-lambda";
import {Duration} from "aws-cdk-lib";
import {REGION} from "./constants";

export const createFunctions = (scope: Construct) => (tableName: string) => {
    const chargerAlert = new Function(scope, 'ChargerAlert', {
        code: Code.fromAsset('../charger-alert.zip'),
        runtime: Runtime.PROVIDED_AL2,
        handler: 'some.handler',
        timeout: Duration.seconds(30),
        memorySize: 1024,
        environment: {
            REGION,
            TABLE: tableName,
        },
    });

    return {
        chargerAlert,
    }
};
