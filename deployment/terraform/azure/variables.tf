variable "azure_tenant_id" {
  type        = string
  description = "The tenant id for the project"
}

variable "azure_subscription_id" {
  type = string
}

# variable "tags" {
#   description = "custom tags for resources"
#   type        = map(string)  # NOT list(string)
#   default     = {}           # empty map by default
# }
