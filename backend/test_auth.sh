#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

BASE_URL="http://localhost:3000"

echo -e "${YELLOW}=== TaskForge Authentication Tests ===${NC}\n"

# Test 1: Register a new user
echo -e "${YELLOW}Test 1: Register new user${NC}"
REGISTER_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "SecurePass123",
    "first_name": "Test",
    "last_name": "User"
  }')

echo "$REGISTER_RESPONSE" | jq .

# Extract token from registration response
TOKEN=$(echo "$REGISTER_RESPONSE" | jq -r '.token')

if [ "$TOKEN" != "null" ] && [ -n "$TOKEN" ]; then
  echo -e "${GREEN}✓ Registration successful${NC}\n"
else
  echo -e "${RED}✗ Registration failed${NC}\n"
  exit 1
fi

# Test 2: Try to register same user again (should fail)
echo -e "${YELLOW}Test 2: Register duplicate user (should fail)${NC}"
DUPLICATE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "SecurePass123",
    "first_name": "Test",
    "last_name": "User"
  }')

echo "$DUPLICATE_RESPONSE" | jq .

if echo "$DUPLICATE_RESPONSE" | grep -q "already registered"; then
  echo -e "${GREEN}✓ Duplicate registration correctly rejected${NC}\n"
else
  echo -e "${RED}✗ Duplicate registration should have failed${NC}\n"
fi

# Test 3: Login with correct credentials
echo -e "${YELLOW}Test 3: Login with correct credentials${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "SecurePass123"
  }')

echo "$LOGIN_RESPONSE" | jq .

LOGIN_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')

if [ "$LOGIN_TOKEN" != "null" ] && [ -n "$LOGIN_TOKEN" ]; then
  echo -e "${GREEN}✓ Login successful${NC}\n"
else
  echo -e "${RED}✗ Login failed${NC}\n"
  exit 1
fi

# Test 4: Login with incorrect password
echo -e "${YELLOW}Test 4: Login with incorrect password (should fail)${NC}"
WRONG_PASS_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "WrongPassword123"
  }')

echo "$WRONG_PASS_RESPONSE" | jq .

if echo "$WRONG_PASS_RESPONSE" | grep -q "Invalid credentials"; then
  echo -e "${GREEN}✓ Wrong password correctly rejected${NC}\n"
else
  echo -e "${RED}✗ Wrong password should have failed${NC}\n"
fi

# Test 5: Access protected endpoint without token
echo -e "${YELLOW}Test 5: Access protected endpoint without token (should fail)${NC}"
NO_TOKEN_RESPONSE=$(curl -s -X GET "${BASE_URL}/api/auth/me")

echo "$NO_TOKEN_RESPONSE"

if echo "$NO_TOKEN_RESPONSE" | grep -q "authorization"; then
  echo -e "${GREEN}✓ Correctly rejected request without token${NC}\n"
else
  echo -e "${RED}✗ Should have rejected request without token${NC}\n"
fi

# Test 6: Access protected endpoint with valid token
echo -e "${YELLOW}Test 6: Access protected endpoint with valid token${NC}"
ME_RESPONSE=$(curl -s -X GET "${BASE_URL}/api/auth/me" \
  -H "Authorization: Bearer ${LOGIN_TOKEN}")

echo "$ME_RESPONSE" | jq .

if echo "$ME_RESPONSE" | grep -q "test@example.com"; then
  echo -e "${GREEN}✓ Successfully accessed protected endpoint${NC}\n"
else
  echo -e "${RED}✗ Failed to access protected endpoint${NC}\n"
  exit 1
fi

# Test 7: Access protected endpoint with invalid token
echo -e "${YELLOW}Test 7: Access protected endpoint with invalid token (should fail)${NC}"
INVALID_TOKEN_RESPONSE=$(curl -s -X GET "${BASE_URL}/api/auth/me" \
  -H "Authorization: Bearer invalid_token_12345")

echo "$INVALID_TOKEN_RESPONSE"

if echo "$INVALID_TOKEN_RESPONSE" | grep -q "Invalid token"; then
  echo -e "${GREEN}✓ Correctly rejected invalid token${NC}\n"
else
  echo -e "${RED}✗ Should have rejected invalid token${NC}\n"
fi

echo -e "${GREEN}=== All tests completed! ===${NC}"
