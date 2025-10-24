
# Output the zone ID so other resources (like private endpoints) can reference it
output "sql_private_dns_zone_id" {
  value = azurerm_private_dns_zone.sql_private_dns_zone.id
}