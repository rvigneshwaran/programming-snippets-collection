#!/bin/sh

# Creating the array with the Index
Hogwarts[0]="Hermione Granger"
Hogwarts[1]="Ron Weasley"
Hogwarts[2]="Severus Snape"
Hogwarts[3]="Sirius Black"
Hogwarts[4]="Rubeus Hagrid"
Hogwarts[5]="Minerva McGonagall"

# printing the first index of the array
echo "First Poke: ${Hogwarts[0]}"

# Printing the seconf index of the array
echo "Second Poke: ${Hogwarts[1]}"

# acess all items in the array 
echo "Bringing up everyone from the school :: ${Hogwarts[*]}"
echo "Again Bringing up everyone from the school :: ${Hogwarts[@]}"