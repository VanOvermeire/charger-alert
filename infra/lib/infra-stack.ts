import {Stack, StackProps} from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {createFunctions} from "./modules/functions";
import {buildHttpApiWithLambdas} from "./modules/api";

export class InfraStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const { chargerAlert } = createFunctions(this);

    const httpApi = buildHttpApiWithLambdas(this)(chargerAlert);
  }
}
