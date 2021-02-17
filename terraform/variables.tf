variable "region" {
  default = "us-east-1"
}

variable "account" {
  default = "local"
}

variable "access_key" {
  default = "fake_access_key"
}

variable "secret_key" {
  default = "fake_secret_key"
}

variable "s3_force_path_style" {
  default = true
}

variable "skip_credentials_validation" {
  default = true
}

variable "skip_metadata_api_check" {
  default = true
}

variable "skip_requesting_account_id" {
  default = true
}

variable "localstack_url" {
  default = "http://localhost:4566"
}