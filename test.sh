#!/bin/bash

# URL of the API endpoint
BASE_URL="http://127.0.0.1:8080"

test_insert_dataset() {
    local url="${BASE_URL}/insert"
    local data='{"name": "Test", "description": "Test description"}'
    local response=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$url" -H "Content-Type: application/json" -d "$data")

    if [ "$response" -eq 200 ]; then
        echo "Insert dataset test passed"
    else
        echo "Insert dataset test failed with status code $response"
    fi
}

test_get_dataset() {
    local document_id="$1"
    local url="${BASE_URL}/get/${document_id}"
    local response=$(curl -s -o /dev/null -w "%{http_code}" -X GET "$url")

    if [ "$response" -eq 200 ]; then
        echo "Get dataset test passed"
    else
        echo "Get dataset test failed with status code $response"
    fi
}

# Run the tests
test_insert_dataset
test_get_dataset "1"

# temp functions to be implemented

temp_update_dataset() {
    local document_id="$1"
    local url="${BASE_URL}/update/${document_id}"
    local data='{"name": "Updated Test", "description": "Updated description"}'
    local response=$(curl -s -o /dev/null -w "%{http_code}" -X PUT "$url" -H "Content-Type: application/json" -d "$data")

    if [ "$response" -eq 200 ]; then
        echo "Update dataset test passed"
    else
        echo "Update dataset test failed with status code $response"
    fi
}

test_delete_dataset() {
    local document_id="$1"
    local url="${BASE_URL}/delete/${document_id}"
    local response=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$url")

    if [ "$response" -eq 200 ]; then
        echo "Delete dataset test passed"
    else
        echo "Delete dataset test failed with status code $response"
    fi
}

# Uncomment to run these tests
# temp_update_dataset "1"
# test_delete_dataset "1"
