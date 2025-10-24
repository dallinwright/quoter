# Primary logical server in West US
resource "azurerm_mssql_server" "primary" {
  name                         = "sql-${var.env}-westus"
  resource_group_name          = var.resource_group_name
  location                     = "westus"
  version                      = var.sql_version
  administrator_login          = var.sql_admin_login
  administrator_login_password = var.sql_admin_password

  public_network_access_enabled = false

  tags = var.tags
}

# Secondary logical server in East US
resource "azurerm_mssql_server" "secondary" {
  name                         = "sql-${var.env}-eastus"
  resource_group_name          = var.resource_group_name
  location                     = "eastus"
  version                      = var.sql_version
  administrator_login          = var.sql_admin_login
  administrator_login_password = var.sql_admin_password

  public_network_access_enabled = false

  tags = var.tags
}

# Primary Hyperscale database on the primary (West US) server
# Hyperscale as the "new" wasy as it decouples compute, storage, and log layers.
resource "azurerm_mssql_database" "db" {
  name           = var.db_name
  server_id      = azurerm_mssql_server.primary.id
  sku_name       = "HS_Gen5_8" # Hyperscale SKU (example — adjust cores as needed)
  max_size_gb    = var.max_size_gb
  zone_redundant = false

  tags = var.tags
}

# Auto-failover group to replicate Hyperscale DB to secondary (East US)
resource "azurerm_mssql_failover_group" "failover_group" {
  name      = var.failover_group_name
  server_id = azurerm_mssql_server.primary.id

  databases = [azurerm_mssql_database.db.id]

  partner_server {
    id = azurerm_mssql_server.secondary.id
  }

  read_write_endpoint_failover_policy {
    mode          = "Automatic"
    grace_minutes = var.grace_minutes
  }

  tags = var.tags
}

# Private endpoint for primary (West US)
resource "azurerm_private_endpoint" "sql_westus_private_endpoint" {
  name                = "private-endpoint-sql-${var.env}-westus"
  location            = "westus"
  resource_group_name = azurerm_mssql_server.primary.resource_group_name
  subnet_id           = var.private_endpoint_subnet_id_westus

  private_service_connection {
    name                           = "psc-sql-${var.env}-westus"
    private_connection_resource_id = azurerm_mssql_server.primary.id
    subresource_names              = ["sqlServer"]
    is_manual_connection           = false
  }

  private_dns_zone_group {
    name                 = "sql-pl-zone-group-west"
    private_dns_zone_ids = [var.sql_private_dns_zone_id]
  }

  tags = var.tags
}

# Private endpoint for secondary (East US)
resource "azurerm_private_endpoint" "sql_eastus_private_endpoint" {
  name                = "private-endpoint-sql-${var.env}-eastus"
  location            = "eastus"
  resource_group_name = azurerm_mssql_server.secondary.resource_group_name
  subnet_id           = var.private_endpoint_subnet_id_eastus

  private_service_connection {
    name                           = "psc-sql-${var.env}-eastus"
    private_connection_resource_id = azurerm_mssql_server.secondary.id
    subresource_names              = ["sqlServer"]
    is_manual_connection           = false
  }

  private_dns_zone_group {
    name                 = "sql-pl-zone-group-east"
    private_dns_zone_ids = [var.sql_private_dns_zone_id]
  }

  tags = var.tags
}