terraform {
  backend "remote" {
    organization = "boop-ninja"
    workspaces {
      name = "iac-name-bot"
    }
  }
}

