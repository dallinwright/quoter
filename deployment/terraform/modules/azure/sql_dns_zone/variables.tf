variable "tags" {
  description = "A map of tags to assign to all resources."
  type        = map(string)
}

variable "resource_group_name_shared" {
  description = "The name of the shared/global resource group (used for networking and DNS)."
  type        = string
}

variable "resource_group_name_eastus" {
  description = "The name of the resource group that contains East US resources."
  type        = string
}

variable "resource_group_name_westus" {
  description = "The name of the resource group that contains West US resources."
  type        = string
}

variable "vnet_id_eastus" {
  description = "The resource ID of the virtual network in East US."
  type        = string
}

variable "vnet_id_westus" {
  description = "The resource ID of the virtual network in West US."
  type        = string
}
