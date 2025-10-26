variable "sql_admin_login" {
  description = "SQL administrator login"
  type        = string
  default     = "adminuser"
}

variable "sql_admin_password" {
  description = "SQL administrator password"
  type        = string
  sensitive   = true
}

variable "failover_group_name" {
  description = "Failover group name"
  type        = string
  default     = "failover-group-appdb"
}

variable "tags" {
  description = "Map of tags to apply to all resources"
  type        = map(string)
}

variable "resource_group_name" {
  description = "Resource group for SQL servers and private endpoints"
  type        = string
}

variable "env" {
  description = "Environment short name used in resource naming (e.g., dev, qa, prod)"
  type        = string
}

variable "private_endpoint_subnet_id_westus" {
  description = "Subnet ID for the West US SQL private endpoint"
  type        = string
}

variable "private_endpoint_subnet_id_eastus" {
  description = "Subnet ID for the East US SQL private endpoint"
  type        = string
}

variable "sql_private_dns_zone_id" {
  description = "Private DNS zone ID for privatelink.database.windows.net"
  type        = string
}

variable "sql_version" {
  description = "Major SQL Server version for logical servers"
  type        = string
  default     = "12.0"
}

variable "max_size_gb" {
  description = "Maximum database size (GB) for the Hyperscale database"
  type        = number
  default     = 20
}

variable "grace_minutes" {
  description = "Time in minutes to wait before automatic failover in the failover group"
  type        = number
  default     = 60 # The minimum azure allows
}
