variable "location" {
  description = "Azure region for the resources"
  type        = string
  default     = "eastus"
}

variable "resource_group_name" {
  description = "Name of the resource group"
  type        = string
  default     = "rg-network-demo"
}

variable "vnet_name" {
  description = "Name of the vnet to manage"
  type = string
}

variable "cidr_block" {
  description = "CIDR block for vnet"
  type = string
}

variable "subnet_count" {
  description = "How many /24 subnets to create within the /16"
  type        = number
  default     = 3
}

variable "subnet_newbits" {
  description = "Newbits to derive subnet size from the VNet CIDR (8 turns /16 into /24)"
  type        = number
  default     = 8
}
