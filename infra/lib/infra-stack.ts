import {Stack, StackProps} from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {createFunctions} from "./modules/functions";
import {createHttpApi} from "./modules/api";
import {createDatabase} from "./modules/database";
import {createCron} from "./modules/cron";
import {DEFAULT_AWS_REGION, DEFAULT_SOURCE_EMAIL} from "./modules/constants";

export class InfraStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const region = process.env.AWS_REGION || DEFAULT_AWS_REGION;
    const source_email = process.env.SOURCE_EMAIL || DEFAULT_SOURCE_EMAIL;

    const table = createDatabase(this);
    const { alertAdder, chargeChecker, getChargers } = createFunctions(this)(table, region, source_email);
    createHttpApi(this)(alertAdder, getChargers, region);
    createCron(this)(chargeChecker);
  }
}
