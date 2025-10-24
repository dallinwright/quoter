resource "azurerm_resource_group" "rg_network_shared" {
  name     = "${var.env}_rg_network_shared"
  location = "eastus" # even though specific in a location, this is meant to hold global resources like AWS Route53

  tags = merge(
    var.tags,
    {
      location = "eastus"
    }
  )
}

# Example of how to setup vnets with subnets by resource group and region.
module "vnet_compute_eastus" {
  source = "../modules/azure/vnet"

  vnet_name           = "vnet-compute-eastus"
  resource_group_name = azurerm_resource_group.rg_eastus.name
  location            = "eastus"

  cidr_block     = "10.0.0.0/16"
  subnet_count   = 3 # creates 3 subnets
  subnet_newbits = 8 # /16 + 8 = /24 subnets
}

module "vnet_compute_westus" {
  source = "../modules/azure/vnet"

  vnet_name           = "vnet-compute-eastus"
  resource_group_name = azurerm_resource_group.rg_westus.name
  location            = "westus"

  cidr_block     = "10.10.0.0/16"
  subnet_count   = 3 # creates 3 subnets
  subnet_newbits = 8 # /16 + 8 = /24 subnets
}
