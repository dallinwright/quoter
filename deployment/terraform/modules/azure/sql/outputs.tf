output "sql_primary_fqdn" {
  value = azurerm_mssql_server.primary.fully_qualified_domain_name
}

output "sql_secondary_fqdn" {
  value = azurerm_mssql_server.secondary.fully_qualified_domain_name
}

output "failover_group_readwrite_listener" {
  description = "Read/write listener FQDN (moves with primary)"
  value       = azurerm_mssql_failover_group.fg.read_write_endpoint
}

output "failover_group_readonly_listener" {
  description = "Read-only listener FQDN (routes to secondary)"
  value       = azurerm_mssql_failover_group.fg.read_only_endpoint
}