import {Construct} from "constructs";
import {CfnOutput, Duration} from "aws-cdk-lib";
import {API_VERSION} from "./constants";
import {Function} from "aws-cdk-lib/aws-lambda";
import {CorsHttpMethod, HttpApi, HttpMethod} from "@aws-cdk/aws-apigatewayv2-alpha";
import {HttpLambdaIntegration} from "@aws-cdk/aws-apigatewayv2-integrations-alpha";

export const createHttpApi = (scope: Construct) => (chargerAlert: Function, getChargers: Function, region: string): HttpApi => {
    const httpApi = new HttpApi(scope, 'ChargerAlertApi', {
        // useful for web testing //
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
    const getChargersIntegration = new HttpLambdaIntegration('GetChargers', getChargers);

    httpApi.addRoutes({
        path: `${API_VERSION}/alert`,
        methods: [HttpMethod.POST],
        integration: alertIntegration,
    });

    httpApi.addRoutes({
        path: `${API_VERSION}/chargers`,
        methods: [HttpMethod.GET],
        integration: getChargersIntegration,
    });

    new CfnOutput(scope, 'apiUrl', {
        value: `https://${httpApi.httpApiId}.execute-api.${region}.amazonaws.com`,
        description: 'The url of the http api',
    });

    return httpApi;
}
