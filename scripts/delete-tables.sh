#!/bin/bash

. env/.aws.env

for t in $(./.list-tables.sh); do
	aws dynamodb delete-table \
		--table-name "$t" \
		--region "$REGION" \
		--endpoint-url "$ENDPOINT" \
		--cli-connect-timeout 6000 2>&1 1>/dev/null
done
