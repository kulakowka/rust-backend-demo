#!/bin/bash

# Test AI endpoints

echo "=== Testing Gemini AI Integration ==="
echo ""

BASE_URL="http://127.0.0.1:3001"

echo "1. Testing /ai/chat endpoint..."
curl -X POST "$BASE_URL/ai/chat" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Hello! What is Rust programming language?",
    "history": []
  }'
echo -e "\n"

echo "2. Testing /ai/generate endpoint..."
curl -X POST "$BASE_URL/ai/generate" \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Write a haiku about coding in Rust"
  }'
echo -e "\n"

echo "3. Testing /ai/chat/stream endpoint..."
curl -X POST "$BASE_URL/ai/chat/stream" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Tell me a short story about a robot"
  }' \
  -N
echo -e "\n"

echo "=== All tests completed ==="
