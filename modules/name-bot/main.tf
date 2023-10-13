
resource "helm_release" "i" {
  chart = "../../deploy"
  name  = "name-bot"

  set_sensitive {
    name  = "secret.token"
    value = var.token
  }

  set_sensitive {
    name  = "secret.user_id"
    value = var.user_id
  }

  set_sensitive {
    name  = "secret.super_user_id"
    value = var.super_user_id
  }

  set {
      name  = "image.tag"
      value = "latest"
  }
}

