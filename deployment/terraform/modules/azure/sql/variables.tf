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

variable "db_name" {
  description = "Primary database name"
  type        = string
  default     = "appdb"
}

variable "failover_group_name" {
  description = "Failover group name"
  type        = string
  default     = "failover-group-appdb"
}

variable "tags" {
  type = map(string)
}

variable "resource_group_name" {
  type = string
}

variable "env" {
  type = string
}

variable "grace_minutes" {
  default = 15 # how long before azure promotes secondary to primary
  type = number
}

variable "max_size_gb" {
  description = "Max size for the DB"
  type        = number
  default     = 1024
}

variable "sql_version" {
  description = "SQL server version"
  type        = string
  default     = "12.0"
}

variable "private_endpoint_subnet_id_westus" {
  type = string
}

variable "private_endpoint_subnet_id_eastus" {
  type = string
}

variable "sql_private_dns_zone_id" {
  type = string
}
