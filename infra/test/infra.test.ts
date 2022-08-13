import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as Infra from '../lib/infra-stack';

describe('Charger infrastructure', () => {
    it('should create a Lambda with a custom runtime', () => {
        const app = new cdk.App();

        const stack = new Infra.InfraStack(app, 'TestStack');

        const template = Template.fromStack(stack);

        template.hasResourceProperties('AWS::Lambda::Function', {
            Runtime: "provided.al2"
        });
    });

    it('should create an API with lambda integration and alert route', () => {
        const app = new cdk.App();

        const stack = new Infra.InfraStack(app, 'TestStack');

        const template = Template.fromStack(stack);

        template.hasResourceProperties('AWS::ApiGatewayV2::Api', {});
        template.hasResourceProperties('AWS::ApiGatewayV2::Integration', {
            IntegrationType: 'AWS_PROXY'
        });
        template.hasResourceProperties('AWS::ApiGatewayV2::Route', {
            RouteKey: 'POST /v1/alert'
        })
    });
});



