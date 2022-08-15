import {Stack, StackProps} from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {createFunctions} from "./modules/functions";
import {createHttpApi} from "./modules/api";
import {createDatabase} from "./modules/database";

export class InfraStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const table = createDatabase(this);
    const { chargerAlert } = createFunctions(this)(table);
    const httpApi = createHttpApi(this)(chargerAlert);
  }
}
