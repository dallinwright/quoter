locals {
  # last index within the generated subnets (0-based). This is used to assign the last subnet as privat
  # e endpoint policy disabled.
  private_endpoint_last_index = var.subnet_count - 1

  # Convenience locals/outputs to reference the private endpoint subnet
  # Basically it gets the value in the last index, coerces it to a string then looks it up from the subnet
  # list for the value
  private_endpoint_subnet_key = tostring(local.private_endpoint_last_index)
  private_endpoint_subnet_id  = azurerm_subnet.subnets[local.private_endpoint_subnet_key].id
  private_endpoint_subnet_name = azurerm_subnet.subnets[local.private_endpoint_subnet_key].name
}