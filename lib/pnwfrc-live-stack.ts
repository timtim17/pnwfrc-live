import { Certificate } from 'aws-cdk-lib/aws-certificatemanager';
import * as cdk from 'aws-cdk-lib';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { Construct } from 'constructs';
import { FunctionUrlOrigin } from 'aws-cdk-lib/aws-cloudfront-origins';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { RustFunction } from 'cargo-lambda-cdk';

interface PNWFRCLiveStackProps extends cdk.StackProps {
    certificateArn: string;
}

export class PNWFRCLiveStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: PNWFRCLiveStackProps) {
    super(scope, id, props);

    // create Lambda
    const rocketFn = new RustFunction(this, 'RocketRouterFunction', {
        manifestPath: './lambda',
        architecture: lambda.Architecture.ARM_64,
        logRetention: RetentionDays.THREE_DAYS,
        bundling: {
            cargoLambdaFlags: [
                '-F',
                'lambda',
            ],
        },
    });
    const rocketFnUrl = rocketFn.addFunctionUrl({
        authType: lambda.FunctionUrlAuthType.NONE,
    });
    new cdk.CfnOutput(this, 'RocketFnUrl', { value: rocketFnUrl.url });
    const cfDistribution = new cloudfront.Distribution(this, 'CloudfrontDistribution', {
        defaultBehavior: {
            origin: new FunctionUrlOrigin(rocketFnUrl),
            viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
            allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD,
            cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
            originRequestPolicy: cloudfront.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
        },
        domainNames: ['pnwfrc.live', 'www.pnwfrc.live',],
        certificate: Certificate.fromCertificateArn(this, 'ImportedCert', props.certificateArn),
        webAclId: 'arn:aws:wafv2:us-east-1:267253737119:global/webacl/CreatedByCloudFront-e2e3b930/0d8b55a2-5de7-4f65-aa4f-0849fd31330f',   // Cloudfront Free plan created in console
    });
    new cdk.CfnOutput(this, 'DistributionDomainName', {
        value: cfDistribution.distributionDomainName,
    });

    this.tags.setTag('AppManagerCFNStackKey', this.stackName);
  }
}
