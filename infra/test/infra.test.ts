import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as Infra from '../lib/infra-stack';

test('Creates a Lambda with a custom runtime', () => {
    const app = new cdk.App();

    const stack = new Infra.InfraStack(app, 'TestStack');

    const template = Template.fromStack(stack);

    template.hasResourceProperties('AWS::Lambda::Function', {
        Runtime: "provided.al2FAKE"
    });
});
