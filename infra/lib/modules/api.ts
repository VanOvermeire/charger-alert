import {Construct} from "constructs";
import {CfnOutput} from "aws-cdk-lib";
import {API_VERSION} from "./constants";
import {Function} from "aws-cdk-lib/aws-lambda";
import {HttpApi, HttpMethod} from "@aws-cdk/aws-apigatewayv2-alpha";
import {HttpLambdaIntegration} from "@aws-cdk/aws-apigatewayv2-integrations-alpha";

export const createHttpApi = (scope: Construct) => (chargerAlert: Function, getChargers: Function, region: string): HttpApi => {
    const httpApi = new HttpApi(scope, 'ChargerAlertApi');

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
        description: 'The url of the api',
    });

    return httpApi;
}
