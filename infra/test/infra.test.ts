import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as Infra from '../lib/infra-stack';

/**
 * Tests requires fake zips for the lambdas to run, also see github-deploy.yml
  */

const getTemplate = (): Template => {
    const app = new cdk.App();

    const stack = new Infra.InfraStack(app, 'TestStack');

    return Template.fromStack(stack);
};

describe('Charger infrastructure', () => {
    it('should create Lambdas with a custom runtime and permission to read / write to dynamo', () => {
        const template = getTemplate();

        template.resourceCountIs('AWS::Lambda::Function', 3);
        template.hasResourceProperties('AWS::Lambda::Function', {
            Runtime: "provided.al2"
        });
        template.hasResourceProperties('AWS::IAM::Policy', {
            PolicyDocument: {
                Statement: [
                    {
                        Action: [
                            "dynamodb:BatchWriteItem",
                            "dynamodb:PutItem",
                            "dynamodb:UpdateItem",
                            "dynamodb:DeleteItem",
                            "dynamodb:DescribeTable"
                        ]
                    }
                ]
            }
        });
        template.hasResourceProperties('AWS::IAM::Policy', {
            PolicyDocument: {
                Statement: [
                    {
                        Action: [
                            "ses:SendEmail",
                            "SES:SendRawEmail"
                        ],
                        Effect: "Allow",
                        Resource: "*"
                    },
                    {
                        Action: [
                            "dynamodb:BatchGetItem",
                            "dynamodb:GetRecords",
                            "dynamodb:GetShardIterator",
                            "dynamodb:Query",
                            "dynamodb:GetItem",
                            "dynamodb:Scan",
                            "dynamodb:ConditionCheckItem",
                            "dynamodb:BatchWriteItem",
                            "dynamodb:PutItem",
                            "dynamodb:UpdateItem",
                            "dynamodb:DeleteItem",
                            "dynamodb:DescribeTable"
                        ],
                        Effect: "Allow"
                    }
                ]
            }
        });
    });

    it('should create an API with both an alert and chargers endpoint', () => {
        const template = getTemplate();

        template.hasResourceProperties('AWS::ApiGatewayV2::Api', {});
        template.hasResourceProperties('AWS::ApiGatewayV2::Integration', {
            IntegrationType: 'AWS_PROXY'
        });
        template.hasResourceProperties('AWS::ApiGatewayV2::Route', {
            RouteKey: 'POST /v1/alert'
        })
        template.hasResourceProperties('AWS::ApiGatewayV2::Route', {
            RouteKey: 'GET /v1/chargers'
        })
    });

    it('should create a database', () => {
        const template = getTemplate();

        template.hasResourceProperties('AWS::DynamoDB::Table', {});
    });

    it('should create a rule', () => {
        const template = getTemplate();

        template.hasResourceProperties('AWS::Events::Rule', {});
    });
});
