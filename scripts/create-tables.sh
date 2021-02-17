#!/bin/bash

. env/.aws.env

for t in json/*_table.json; do
	aws dynamodb create-table \
		--region "$REGION" \
		--cli-input-json "file://$t" \
		--endpoint-url "$ENDPOINT" 2>&1 1>/dev/null
done
