resource "azurerm_resource_group" "rg_eastus" {
  name     = "${var.env}_rg_eastus"
  location = "eastus"

  tags = merge(
    var.tags,
    {
      location = "eastus"
    }
  )
}

resource "azurerm_resource_group" "rg_westus" {
  name     = "${var.env}_rg_westus"
  location = "westus"

  tags = merge(
    var.tags,
    {
      location = "westus"
    }
  )
}

# For Hyperscale setup see databases.tf
# For VNET setup, private endpoints, subnets, etc. see networking.tf
# For AKS see a would be aks.tf. That is not trivial as entra-id is preferably needed, keyvault,
# scalesets, etc along with RBAC. That is where the app would go.