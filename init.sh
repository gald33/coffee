#!/bin/zsh

clear

# Prompt for PRICE
# echo -n "Enter price: "
# read PRICE
# temporary defaults
PRICE="274"
FRICTION="0.05"

# Export the PRICE and FRICTION variables for future use
export PRICE
export FRICTION

# Reset simulation and create an account
resim reset
output=$(resim new-account)

# Extract the package from the output
ACCOUNT=$(echo "$output" | grep -o 'account_sim[a-zA-Z0-9]\+')

# Export the account variable for future use
export ACCOUNT
echo "Account = $ACCOUNT" 

# Publish package
output=$(resim publish .)

# Extract the package from the output
PACKAGE=$(echo "$output" | grep -o 'package_[a-zA-Z0-9]\+')

# Export the PACKAGE variable for future use
export PACKAGE
echo "Package = $PACKAGE" 

# Get XRP resource address                                
output=$(resim show)

# Extract the resource address from the line ending with (XRD)
RESOURCE=$(echo "$output" | grep '(XRD)$' | grep -o 'resource_sim[[:alnum:]]\+')

# Export the RESOURCE variable for future use
export RESOURCE
echo "XRD Resource = $RESOURCE" 


# Initialize component
output=$(resim call-function $PACKAGE Coffee instantiate_coffee_usa $RESOURCE "$PRICE" "$FRICTION")
echo $output

# Extract the resource address from the line ending with (XRD)
# This gives the WRONG component currently
COMPONENT=$(echo "$output" | grep -o 'component_sim[^ ]*' | tail -n 1)

# Export the RESOURCE variable for future use
export COMPONENT
echo "Component = $COMPONENT" 