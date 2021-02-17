resource "aws_dynamodb_table" "chargeback" {
  name         = "Chargeback"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "ID"
  range_key    = "rangeID"

  attribute {
    name = "ID"
    type = "S"
  }

  attribute {
    name = "rangeID"
    type = "S"
  }

  tags = {
    Environment = var.account
  }
}