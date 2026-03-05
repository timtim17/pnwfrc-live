## Deployment
1. **Build Lambda:** `cargo lambda build -r --arm64 -F lambda` in `lambda/`
2. **Deploy CDK:** `npm run cdk deploy` in root directory

## Testing
`cargo lambda watch` in `lambda/` should provide a URL to test against.
