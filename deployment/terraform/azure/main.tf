resource "azurerm_resource_group" "rg_eastus" {
  name     = "${var.env}_rg_eastus"
  location = "eastus"

  tags = merge(
    var.tags,
    {
      location = "eastus"
    }
  )
}

resource "azurerm_resource_group" "rg_westus" {
  name     = "${var.env}_rg_westus"
  location = "westus"

  tags = merge(
    var.tags,
    {
      location = "westus"
    }
  )
}


