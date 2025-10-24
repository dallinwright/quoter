output "vnet_id" {
  value = azurerm_virtual_network.vnet.id
}

output "vnet_name" {
  value = azurerm_virtual_network.vnet.name
}

output "subnet_ids" {
  description = "List of subnet IDs ordered by index"
  value = [
    for i in sort(keys(azurerm_subnet.subnets)) :
    azurerm_subnet.subnets[i].id
  ]
}

output "subnet_names" {
  description = "List of subnet names ordered by index"
  value = [
    for i in sort(keys(azurerm_subnet.subnets)) :
    azurerm_subnet.subnets[i].name
  ]
}

output "subnet_prefixes" {
  description = "Map of subnet name => CIDR prefix"
  value = {
    for k, s in azurerm_subnet.subnets :
    s.name => s.address_prefixes[0]
  }
}

output "private_endpoint_subnet_id" {
  description = "Subnet ID designated for Private Endpoints"
  value       = local.private_endpoint_subnet_id
}

output "private_endpoint_subnet_name" {
  description = "Subnet name designated for Private Endpoints"
  value       = local.private_endpoint_subnet_name
}
