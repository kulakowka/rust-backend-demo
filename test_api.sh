#!/bin/bash

BASE_URL="http://localhost:3005"

echo "1. Creating a user..."
CREATE_RES=$(curl -s -X POST $BASE_URL/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}')
echo $CREATE_RES
USER_ID=$(echo $CREATE_RES | jq -r '.id')
echo "Created User ID: $USER_ID"

echo -e "\n2. Getting all users..."
curl -s $BASE_URL/users | jq .

echo -e "\n3. Getting user by ID..."
curl -s $BASE_URL/users/$USER_ID | jq .

echo -e "\n4. Updating user..."
curl -s -X PUT $BASE_URL/users/$USER_ID \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Updated"}' | jq .

echo -e "\n5. Deleting user..."
curl -s -X DELETE $BASE_URL/users/$USER_ID | jq .

echo -e "\n6. Verifying deletion (should fail)..."
curl -s -w "%{http_code}" $BASE_URL/users/$USER_ID
