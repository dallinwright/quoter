# Quoter Demo

Welcome to the demo quoter API. For specifics reference additional project documentation links:

* [Terraform Documentation](./deployment/README.md)
* [API Documentation](./bins/api/README.md)
* [Database Documentation](./migrations/README.md)

### Description
Public web application that connects to an Azure SQL database. The database should be seeded with a list of famous quotes, and when the site is accessed, it should query the database for a random quote and display it.

### Requirements
- Treat all data as critical PII
- The application must be highly available
- Everything should be hosted in Azure, and provisioned using Terraform

### Where was ChatGPT used?

Primarily to assist me in the database setup as this was the first time I used Mssql and it is not as easy as PostgresQL due to licensing issues with Mssql and open source utils.

I also used it to assist in the function documentation and terraform descriptions on variables.

### Approach

High level, It is a globally deployable stateless web application packed inside docker, deployed to Azure Kubernetes Service (AKS), and connected to an Azure SQL database. Components like caching, etc. are not included in the demo for the sake of simplicity and for the short deadline but would be included in a real world application.

#### Treat all data as critical PII

Requirements for PII are data encryption at rest, in transit, access logging, along with principal of the least privilege access controls. I solved for these in a few distinct ways.

1. **Encryption at rest**: The database is by default encrypted at rest using Azure Key Vault which is a standard included feature of Azure SQL block storage. You can either use Azure provided keys or bring your own keys and store in Key Vault.

2. **Encryption in transit**: The database is encrypted in transit using mTLS. Note how there is no certificate nor TLS server setup in the application itself. This is by design, storing the certificate in Key Vault is a standard practice and mounted via sidecar which does mTLS in a service mesh in AKS (Kubernetes + Istio + Envoy), and storing certificates in the container is a bad practice, for example if you have 10,000 containers and revoke a certificate anywhere in the PKI chain, you need to redeploy all 10,000 containers.

3. **Access Control** The application is secured using Azure AD and RBAC (or would be in the real world). To demonstrate how the application would do this without rolling the Oauth2 Infrastructure, EntraID, KeyVault, Authentication microservice and JWT middleware, I included where and how it would be done in the code. Thus, there is a pseudo header called `X-User-Id` that is used for row-based access control in Azure SQL contexts that show how it would be done, where that is a customer JWT claim via oauth validated and provided by EntraID. When a user queries for the data, no one not even the admin can see the data that does not belong to them. Additional steps could be taken to further secure the data for example by using AES encryption wth a nonce set on PII data stored in the database. This would be done in the application code, and stored in the database.

4. **Logging** The application would log if deployed to Azure App Insights, thus every request would be logged. Infrastructure access logs are also logged to Azure Storage.

The Azure EntraID, KeyVault, AKS terraform is not included in the demo as to do it right is a large project. The idea was to show how it would be structured and done via the `deployment` directory and include the terraform code to provision the baseline Azure SQL, networking, and resource groups.

#### The application must be highly available

1. **Application Load Balancer**: Using Traffic Manager with Azure DNS, or via Azure Front Door, the application would load balanced across multiple regions with round-robin routing. This handles the ingress layer to route to the application replica sets.
2. **Application Replica Sets**: The application would be deployed in 1+N replicas in a multi-region AKS cluster. This handles the application layer.
3. **Database Replica Sets**: The database would be deployed in 1+N replicas in a multi-region Azure SQL database. This handles the database layer.

The application itself is fully stateless and deployed via Docker Containers. You can see the multi-stage docker image in the `docker` directory. The `helm`, `flux-cd` and `kustomize` directories would be how I would typically deploy to Kubernetes. This means that not only is it highly available, but it is also highly scalable, regionally isolated, and deployed Active-Active potentially globally so is beyond just HA withing a single region.

#### Everything should be hosted in Azure, and provisioned using Terraform

The terraform code in the `deployment` directory would provision the baseline infrastructure for the demo. I included the code, modules, and readme up to the `terraform apply` step as I did not want to incur the costs associate with actually provisioning the demo. It can be provisioned as is, but critical components like KeyVault, AKS, EntraID would need to be configured.

### Conclusion

Although key, critical components are not included in the demo, the idea is to show how to start the foundational setup to deploy an application to Azure and to start to manage the infrastructure. Attention should be on the key components covered like Terraform, Docker, Azure, Azure SQL, Application design, etc. as they are incomplete but intended to show key areas that would be needed.
