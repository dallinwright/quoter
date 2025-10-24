# Deployment

This is a sample of how you could deploy. In the real world, this would
have versioned, external modules that are not local or directly referenced.

```bash
cd deployment/terraform

terraform workspace new dev
terraform workspace select dev

terraform init -backend-config=./envs/dev/backend.hcl

terraform plan --var-file=./envs/dev/values.tfvars
```
