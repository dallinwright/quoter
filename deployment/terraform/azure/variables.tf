variable "azure_tenant_id" {
  type        = string
  description = "The tenant id for the project"
}

variable "azure_subscription_id" {
  type = string
}

variable "env" {
  type = string
}

variable "tags" {
  description = "custom tags for resources"
  type        = map(string) # NOT list(string)
  default     = {}          # empty map by default
}

# For demo purposes, in the real world use KeyVault. That is a lot to setup though so for now doing this.
variable "sql_admin_password" {
  type      = string
  sensitive = true
}
