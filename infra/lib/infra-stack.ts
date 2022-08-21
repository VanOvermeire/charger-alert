import {Stack, StackProps} from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {createFunctions} from "./modules/functions";
import {createHttpApi} from "./modules/api";
import {createDatabase} from "./modules/database";
import {createCron} from "./modules/cron";
import {DEFAULT_AWS_REGION} from "./modules/constants";

export class InfraStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const region = process.env.AWS_REGION || DEFAULT_AWS_REGION;

    const table = createDatabase(this);
    const { alertAdder, chargeChecker } = createFunctions(this)(table, region);
    createHttpApi(this)(alertAdder, region);
    createCron(this)(chargeChecker);
  }
}
