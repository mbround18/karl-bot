resource "kubernetes_namespace" "i" {
    metadata {
        name = "name-bot"
    }
}


resource "helm_release" "i" {
  depends_on = [kubernetes_namespace.i]

  chart = "${path.root}/charts/name-bot"
  name  = "name-bot"
  namespace = "name-bot"

  set_sensitive {
    name  = "secret.client_id"
    value = var.client_id
  }

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

