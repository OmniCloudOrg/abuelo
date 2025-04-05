#!/bin/bash

# Simple API Test Script for User and Handle Management
# Tests all routes in the API with minimal complexity

# Configuration
API_URL="http://localhost:8000"
USERNAME="test_user_$(date +%s)"  # Generate unique username with timestamp
PASSWORD="test_password"

# Text colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print section headers
print_header() {
    echo -e "\n${BLUE}==== $1 ====${NC}\n"
}

# Function to make API requests and display results
make_request() {
    local method="$1"
    local endpoint="$2"
    local data="$3"
    local description="$4"
    
    echo -e "${BLUE}Testing:${NC} $description"
    echo -e "${BLUE}Request:${NC} $method $API_URL$endpoint"
    
    if [ -n "$data" ]; then
        echo -e "${BLUE}Data:${NC} $data"
        response=$(curl -s -X "$method" "$API_URL$endpoint" \
            -H 'Content-Type: application/json' \
            -d "$data")
    else
        response=$(curl -s -X "$method" "$API_URL$endpoint")
    fi
    
    echo -e "${BLUE}Response:${NC} $response"
    echo ""
    
    # Return the response for further processing if needed
    echo "$response"
}

# 1. Test user creation
print_header "Creating User"
create_response=$(make_request "POST" "/user/create" \
    "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\"}" \
    "Create new user")

# 2. Test user authentication
print_header "User Authentication"
auth_response=$(make_request "POST" "/user/auth" \
    "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\"}" \
    "Authenticate user")

# Simple handle extraction - just get the number
HANDLE=$(echo "$auth_response" | grep -o '"handle":[0-9]*' | grep -o '[0-9]*' | head -1)
if [ -n "$HANDLE" ]; then
    echo -e "Extracted handle: $HANDLE"
fi

# 3. Test get user info
print_header "Get User Info"
make_request "GET" "/user/$USERNAME" "" "Get user information"

# 4. Test get user handles
print_header "Get User Handles"
make_request "GET" "/user/$USERNAME/handles" "" "Get all handles for user"

# 5. Test creating a new handle
print_header "Create New Handle"
create_handle_response=$(make_request "POST" "/user/handle/create" \
    "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\"}" \
    "Create a new handle for user")

# Simple handle extraction - just get the number
NEW_HANDLE=$(echo "$create_handle_response" | grep -o '"handle":[0-9]*' | grep -o '[0-9]*' | head -1)
if [ -n "$NEW_HANDLE" ]; then
    echo -e "New handle created: $NEW_HANDLE"
fi

# 6. Test getting user handles again
print_header "Get User Handles Again"
make_request "GET" "/user/$USERNAME/handles" "" "Get all handles for user (after creating new handle)"

# 7. Test deleting a handle - use the NEW_HANDLE if available, otherwise HANDLE
print_header "Delete Handle"
DELETE_HANDLE=${NEW_HANDLE:-$HANDLE}
if [ -n "$DELETE_HANDLE" ]; then
    make_request "POST" "/user/handle/delete" \
        "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\",\"handle\":$DELETE_HANDLE}" \
        "Delete handle $DELETE_HANDLE"
else
    echo -e "${RED}Cannot test handle deletion - no valid handle available${NC}"
fi

# 8. Test authentication with invalid credentials
print_header "Authentication with Invalid Credentials"
make_request "POST" "/user/auth" \
    "{\"username\":\"$USERNAME\",\"password\":\"wrong_password\"}" \
    "Attempt authentication with wrong password"

# 9. Test handle creation with invalid credentials
print_header "Handle Creation with Invalid Credentials"
make_request "POST" "/user/handle/create" \
    "{\"username\":\"$USERNAME\",\"password\":\"wrong_password\"}" \
    "Attempt to create handle with wrong password"

# 10. Test handle deletion with invalid credentials
print_header "Handle Deletion with Invalid Credentials"
if [ -n "$DELETE_HANDLE" ]; then
    make_request "POST" "/user/handle/delete" \
        "{\"username\":\"$USERNAME\",\"password\":\"wrong_password\",\"handle\":$DELETE_HANDLE}" \
        "Attempt to delete handle with wrong password"
fi

# 11. Test handle deletion with non-existent handle
print_header "Delete Non-existent Handle"
make_request "POST" "/user/handle/delete" \
    "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\",\"handle\":999999}" \
    "Attempt to delete non-existent handle"

# 12. Test get non-existent user
print_header "Get Non-existent User"
make_request "GET" "/user/nonexistent_user" "" "Attempt to get information for non-existent user"

print_header "Test Summary"
echo -e "All API endpoints have been tested!"
echo -e "Test user: $USERNAME"
if [ -n "$HANDLE" ]; then
    echo -e "Initial handle: $HANDLE"
fi
if [ -n "$NEW_HANDLE" ]; then
    echo -e "New handle: $NEW_HANDLE"
fi