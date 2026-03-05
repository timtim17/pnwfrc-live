import * as cdk from 'aws-cdk-lib';
import { Certificate, CertificateValidation } from 'aws-cdk-lib/aws-certificatemanager';
import { Construct } from 'constructs';

export class CertificateStack extends cdk.Stack {
  public readonly certificateArn: string;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, {
        ...props,
        env: {
            ...props?.env,
            region: 'us-east-1',
        }
    });

    const certificate = new Certificate(this, 'Certificate', {
        domainName: 'pnwfrc.live',
        subjectAlternativeNames: ['www.pnwfrc.live'],
        validation: CertificateValidation.fromDns(),
    });
    this.certificateArn = certificate.certificateArn;

    this.tags.setTag('AppManagerCFNStackKey', this.stackName);
  }
}
