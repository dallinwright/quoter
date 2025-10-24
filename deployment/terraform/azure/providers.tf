terraform {
  # backend "azurerm" {} // configuration is provided by the backend.hcl for the env
  backend "local" {} // for local dev without storage account
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.100"
    }
  }
}

provider "azurerm" {
  features {}
  subscription_id = var.azure_subscription_id
  tenant_id       = var.azure_tenant_id
}
