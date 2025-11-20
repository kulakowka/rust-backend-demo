#!/bin/bash

BASE_URL="http://127.0.0.1:3005/graphql"

echo "Testing GraphQL API at $BASE_URL"

# 1. Create User
echo "1. Creating User..."
CREATE_QUERY='mutation { createUser(input: {name: "GraphQL User", email: "graphql@example.com"}) { id name email } }'
PAYLOAD=$(jq -n --arg q "$CREATE_QUERY" '{query: $q}')
curl -s -X POST -H "Content-Type: application/json" -d "$PAYLOAD" $BASE_URL | jq
echo ""

# 2. Get Users
echo "2. Getting Users..."
GET_QUERY='query { users { id name email } }'
PAYLOAD=$(jq -n --arg q "$GET_QUERY" '{query: $q}')
curl -s -X POST -H "Content-Type: application/json" -d "$PAYLOAD" $BASE_URL | jq
echo ""
