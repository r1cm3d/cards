resource "aws_dynamodb_table" "cards" {
  name         = "Cards"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "ID"

  attribute {
    name = "ID"
    type = "S"
  }

  tags = {
    Environment = var.account
  }
}