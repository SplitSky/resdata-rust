#!/bin/bash

# URL of the API endpoint
URL="http://127.0.0.1:8080/insert"

# Make the GET request to the endpoint
response=$(curl -X GET "$URL")

# Print response
echo "Response form the server:"
echo "$response"
