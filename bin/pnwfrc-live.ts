#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { PNWFRCLiveStack } from '../lib/pnwfrc-live-stack';
import { CertificateStack } from '../lib/certificate-stack';

const app = new cdk.App();

const certificateStack = new CertificateStack(app, 'PNWFRCCertificateStack');
new PNWFRCLiveStack(app, 'PNWFRCLiveStack', {
  certificateArn: certificateStack.certificateArn,
  crossRegionReferences: true,
  env: {
    region: 'us-west-2',
  },
});
