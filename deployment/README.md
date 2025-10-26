# Terraform Deployment

This is a sample of how you could deploy. In the real world, this would
have versioned external modules that are not local or directly referenced.

For example, you would have a library of modules in github or via the opinated Terraform Cloud approach of a registry repository per module like this.

### Modules

Example using a versioned module from the Terraform registry, either the cloud registry or a private registry.

```terraform
module "network" {
    source  = "Azure/network/azurerm"
    version = "3.5.0"  # pinned module version
    # source  = "git::https://github.com/Azure/terraform-azurerm-vnet.git?ref=v3.2.0" as an alternative
    
    resource_group_name = "rg-example"
    location            = "eastus"
    address_space       = ["10.0.0.0/16"]
}
```

### Directory Layout

#### Why the specific layout? 

##### Balance between flexibility, scalability, simplicity and time constraint.
It's a compromise between flexibility, scalability, simplicity and the time constraint. Having versioned independent modules
is a good idea, but it's not always practical.

##### Directory per cloud provider.
To demonstrate multiple workspaces and separate isolated states, a specific Azure folder for the cloud infrastructure is used.
Using the current approach you would then for example have a `github`, `cloudflare`, `auth0`, etc. folders per cloud provider
and then split the environments into separate workspaces.

##### State file gotchas.
The reason is that Terraform evaluates the state file sequentially, and on every plan it must evaluate the declared state against the target state. This also means if all state is in a single file, and you have thousands of resources, it can take hours to do a single plan, assuming no errors, state drift, etc. and it must be done every single time. This is not practical, scalable, maintainable or efficient. The current approach is again is to demonstrate the idea, but to not go too far by splitting it to the team, env, platform, etc. level which would be too far in the other direction as well for the given challenge.


```bash
cd deployment/terraform/azure

terraform workspace new dev
terraform workspace select dev

terraform init -backend-config=./envs/dev/backend.hcl

terraform plan --var-file=./envs/dev/values.tfvars
```
