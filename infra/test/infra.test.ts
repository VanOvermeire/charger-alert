import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as Infra from '../lib/infra-stack';

const getTemplate = (): Template => {
    const app = new cdk.App();

    const stack = new Infra.InfraStack(app, 'TestStack');

    return Template.fromStack(stack);
};

describe('Charger infrastructure', () => {
    it('should create a Lambda with a custom runtime and permission to write to dynamo', () => {
        const template = getTemplate();

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
    });

    it('should create an API with lambda integration and alert route', () => {
        const template = getTemplate();

        template.hasResourceProperties('AWS::ApiGatewayV2::Api', {});
        template.hasResourceProperties('AWS::ApiGatewayV2::Integration', {
            IntegrationType: 'AWS_PROXY'
        });
        template.hasResourceProperties('AWS::ApiGatewayV2::Route', {
            RouteKey: 'POST /v1/alert'
        })
    });

    it('should create a database', () => {
        const template = getTemplate();

        template.hasResourceProperties('AWS::DynamoDB::Table', {});
    });
});
