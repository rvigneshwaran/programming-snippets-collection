#!/bin/bash

# Harcoding the variable and printing the instance 
echo "Hello World"

# read the dynamic input and set it in the variable
#echo "What is your Name ?"
#read name
#echo "Hello $name !!!"

# read the input through command line arguements.
echo "The Department of Execution :: $1"

# Print the file name of the current script
echo "File Name of the Current Script : $0"

# Print the first parameter
echo "First Parameter : $1"

# print the second parameter
echo "Second Parameter : $2"

# Print the Quoted Text in the Parameters
echo "Quoted Values: $@"
echo "Quoted Values: $*"

# print the total number of command line arguments supplied to the scripy which is executed 
echo "Total Number of Parameters : $#"

echo "Exit Status of Last command Executed :: $?"

echo "Process Number of Current Shell :: $$"

echo "Process No of the Last back ground Command :: $!"

