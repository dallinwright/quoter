
module "database_dns_zone" {
  source = "../modules/azure/sql_dns_zone"

  resource_group_name_shared = azurerm_resource_group.rg_network_shared.name
  resource_group_name_eastus = azurerm_resource_group.rg_eastus.name
  resource_group_name_westus = azurerm_resource_group.rg_westus.name
  vnet_id_eastus             = module.vnet_compute_eastus.vnet_id
  vnet_id_westus             = module.vnet_compute_westus.vnet_id
  tags                       = var.tags
}

# This is a multi-instance hyperscale mssql setup.
module "database_appdb" {
  source = "../modules/azure/sql"

  db_name                           = "appdb"
  env                               = var.env
  resource_group_name               = azurerm_resource_group.rg_westus.name
  sql_admin_password                = var.sql_admin_password
  private_endpoint_subnet_id_eastus = module.vnet_compute_eastus.private_endpoint_subnet_id
  private_endpoint_subnet_id_westus = module.vnet_compute_westus.private_endpoint_subnet_id
  sql_private_dns_zone_id           = module.database_dns_zone.sql_private_dns_zone_id

  tags = var.tags
}
