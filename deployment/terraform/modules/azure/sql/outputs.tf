output "sql_primary_fqdn" {
  value = azurerm_mssql_server.primary.fully_qualified_domain_name
}

output "sql_secondary_fqdn" {
  value = azurerm_mssql_server.secondary.fully_qualified_domain_name
}
