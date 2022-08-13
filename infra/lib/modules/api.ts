import {Construct} from "constructs";
import {CfnOutput, Duration} from "aws-cdk-lib";
import {API_VERSION, REGION} from "./constants";
import {Function} from "aws-cdk-lib/aws-lambda";
import {CorsHttpMethod, HttpApi, HttpMethod} from "@aws-cdk/aws-apigatewayv2-alpha";
import {HttpLambdaIntegration} from "@aws-cdk/aws-apigatewayv2-integrations-alpha";

export const buildHttpApiWithLambdas = (scope: Construct) => (chargerAlert: Function): HttpApi => {
    const httpApi = new HttpApi(scope, 'ChargerAlertApi', {
        // needed for web testing
        corsPreflight: {
            allowHeaders: ['Authorization'],
            allowMethods: [
                CorsHttpMethod.GET,
                CorsHttpMethod.HEAD,
                CorsHttpMethod.OPTIONS,
                CorsHttpMethod.POST,
            ],
            allowOrigins: ['*'],
            maxAge: Duration.days(10),
        },

    });

    const alertIntegration = new HttpLambdaIntegration('AlertPost', chargerAlert);

    httpApi.addRoutes({
        path: `${API_VERSION}/alert`,
        methods: [HttpMethod.POST],
        integration: alertIntegration,
    });

    new CfnOutput(scope, 'apiUrl', {
        value: `https://${httpApi.httpApiId}.execute-api.${REGION}.amazonaws.com`,
        description: 'The url of the http api',
    });

    return httpApi;
}