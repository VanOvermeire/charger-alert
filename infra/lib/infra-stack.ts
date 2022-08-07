import {Duration, Stack, StackProps} from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {Runtime, Function, Code} from "aws-cdk-lib/aws-lambda";

export class InfraStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const chargerFun = new Function(this, 'ChargerAlert', {
      code: Code.fromAsset('../charger-alert.zip'),
      runtime: Runtime.PROVIDED_AL2,
      handler: 'some.handler',
      timeout: Duration.seconds(30),
      memorySize: 1024,
      environment: {},
    });
  }
}
