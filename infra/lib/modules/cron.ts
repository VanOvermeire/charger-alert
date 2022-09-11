import {Rule, Schedule} from "aws-cdk-lib/aws-events";
import {Duration} from "aws-cdk-lib";
import {LambdaFunction} from "aws-cdk-lib/aws-events-targets";
import {Construct} from "constructs";
import {Function} from "aws-cdk-lib/aws-lambda";

export const createCron = (scope: Construct) => (lambda: Function) =>
    new Rule(scope, 'ChargeCheckerRule', {
        schedule: Schedule.rate(Duration.minutes(5)),
        targets: [new LambdaFunction(lambda)],
        enabled: false, // because it's a demo and I don't want it constantly polling the endpoint
    });
