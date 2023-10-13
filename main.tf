variable "kube_host" {
  type = string
}

variable "kube_crt" {
  type = string
}
variable "kube_key" {
  type = string
}


provider "helm" {
  kubernetes {
    host               = var.kube_host
    client_certificate = base64decode(var.kube_crt)
    client_key         = base64decode(var.kube_key)
    insecure           = true
  }
}

provider "kubernetes" {
  host               = var.kube_host
  client_certificate = base64decode(var.kube_crt)
  client_key         = base64decode(var.kube_key)
  insecure           = true
}

variable "config" {
  type = object({
    token = string
    user_id = string
    super_user_id = string
  })
}


module "name-bot" {
  source = "./modules/name-bot"
  token = var.config.token
  user_id = var.config.user_id
  super_user_id = var.config.super_user_id

}