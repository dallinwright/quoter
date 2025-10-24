# Private DNS Zone for Azure SQL (only one needed, global resource)
resource "azurerm_private_dns_zone" "sql_private_dns_zone" {
  name                = "privatelink.database.windows.net"
  resource_group_name = var.resource_group_name_shared  # central/shared RG for DNS. It is global.

  tags = var.tags
}


# Link the Private DNS Zone to your virtual networks
resource "azurerm_private_dns_zone_virtual_network_link" "sql_dns_zone_link_eastus" {
  name                  = "sql-dns-link-eastus"
  resource_group_name   = var.resource_group_name_eastus
  private_dns_zone_name = azurerm_private_dns_zone.sql_private_dns_zone.name
  virtual_network_id    = var.vnet_id_eastus
  registration_enabled  = false

  tags = var.tags
}

resource "azurerm_private_dns_zone_virtual_network_link" "sql_dns_zone_link_westus" {
  name                  = "sql-dns-link-westus"
  resource_group_name   = var.resource_group_name_westus
  private_dns_zone_name = azurerm_private_dns_zone.sql_private_dns_zone.name
  virtual_network_id    = var.vnet_id_westus
  registration_enabled  = false

  tags = var.tags
}
