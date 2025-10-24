resource "azurerm_virtual_network" "vnet" {
  name                = var.vnet_name
  location            = var.location
  resource_group_name = var.resource_group_name

  address_space = [
    var.cidr_block
  ]
}

# Example how it could be done with for_each dynamically
resource "azurerm_subnet" "subnets" {
  for_each = {
    for i in range(var.subnet_count) :
    i => {
      name = format("%s-subnet-%02d", var.vnet_name, i + 1)
      cidr = cidrsubnet(var.cidr_block, var.subnet_newbits, i)
    }
  }

  name                 = each.value.name
  resource_group_name  = var.resource_group_name
  virtual_network_name = azurerm_virtual_network.vnet.name
  address_prefixes     = [each.value.cidr]

  # Make the last subnet the Private Endpoint subnet
  private_endpoint_network_policies = (
    tonumber(each.key) == local.private_endpoint_last_index ? "Disabled" : "Enabled"
  )
}
