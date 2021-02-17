provider "aws" {
  region                      = var.region
  access_key                  = var.access_key
  s3_force_path_style         = var.s3_force_path_style
  secret_key                  = var.secret_key
  skip_credentials_validation = var.skip_credentials_validation
  skip_metadata_api_check     = var.skip_metadata_api_check
  skip_requesting_account_id  = var.skip_requesting_account_id

  endpoints {
    iam      = var.localstack_url
    s3       = var.localstack_url
    dynamodb = var.localstack_url
    sqs      = var.localstack_url
  }
}

variable "project" {
  default = "cards"
}

resource "aws_iam_user" "cards" {
  name = "cards"
  tags = {
    project = var.project
    env     = var.account
  }
}

resource "aws_iam_policy_attachment" "dynamo_policy_attach" {
  name = "cards-dynamo-attachment"
  users = [
  aws_iam_user.cards.name]
  policy_arn = aws_iam_policy.cards_dynamo_policy.arn
}

resource "aws_iam_policy" "cards_dynamo_policy" {
  name = "cards-dynamo-policy"

  policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "dynamodb",
            "Effect": "Allow",
            "Action": [
                "dynamodb:Scan",
                "dynamodb:Query",
                "dynamodb:DescribeTable",
                "dynamodb:GetItem",
                "dynamodb:GetRecords",
                "dynamodb:PutItem"
            ],
            "Resource": [
                "${aws_dynamodb_table.cards.arn}"
            ]
        }
    ]
}
EOF
}
