terraform {
  cloud {
    organization = "sammye-rs"

    workspaces {
      name = "omnirss-prod"
    }
  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

variable "docker_repo" {
  default = "registry.digitalocean.com/subtlesoftware/omnirss"
}

variable "docker_tag" {}

locals {
  namespace = "prod"

  doppler_secret_name = "doppler-secret"
  doppler_token_secret_name = "doppler-token-secret"
  doppler_token_namespace = "doppler-operator-system"
}

resource "kubernetes_namespace" "namespace" {
  metadata {
    name = local.namespace
  }
}

resource "kubernetes_deployment" "deployment" {
  metadata {
    namespace = local.namespace
    name = "omnirss"
    labels = {
      app = "omnirss"
    }
  }
  spec {
    replicas = 2
    selector {
      match_labels = {
        app = "omnirss"
      }
    }

    template {
      metadata {
        labels = {
          app = "omnirss"
        }
      }
      spec {
        container {
          name = "omnirss"
          image = "${var.docker_repo}:${var.docker_tag}"

          port {
            container_port = 8000
          }

          liveness_probe {
            http_get {
              path = "/health"
              port = 8000
            }
          }

          env {
            name = "ROCKET_BASE_URL"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key = "ROCKET_BASE_URL"
              }
            }
          }
          env {
            name = "ROCKET_LOG_LEVEL"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key  = "ROCKET_LOG_LEVEL"
              }
            }
          }
          env {
            name = "ROCKET_REDIS_ENDPOINT"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key  = "ROCKET_REDIS_ENDPOINT"
              }
            }
          }
          env {
            name = "ROCKET_REDIS_PASSWORD"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key  = "ROCKET_REDIS_PASSWORD"
              }
            }
          }
          env {
            name = "ROCKET_REDIS_PORT"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key  = "ROCKET_REDIS_PORT"
              }
            }
          }
          env {
            name = "ROCKET_SPOTIFY_CLIENT_ID"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key  = "ROCKET_SPOTIFY_CLIENT_ID"
              }
            }
          }
          env {
            name = "ROCKET_SPOTIFY_CLIENT_SECRET"
            value_from {
              secret_key_ref {
                name = local.doppler_secret_name
                key  = "ROCKET_SPOTIFY_CLIENT_SECRET"
              }
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_manifest" "secrets" {
  manifest = {
    apiVersion = "secrets.doppler.com/v1alpha1"
    kind = "DopplerSecret"
    metadata = {
      name = local.doppler_secret_name
      namespace = local.doppler_token_namespace
    }
    spec = {
      tokenSecret = {
        name = local.doppler_token_secret_name
        namespace = local.doppler_token_namespace
      }
      managedSecret = {
        name = local.doppler_secret_name
        namespace = local.namespace
      }
    }
  }
}
