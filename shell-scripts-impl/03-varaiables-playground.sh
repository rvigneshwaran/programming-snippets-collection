#!/bin/sh

# Defining a Scalar variable in shell, it can hold only one value at a time.
first_player="Harry"
echo "The first student to enter the school :: $first_player"

# Unsetting values of the variable
unset first_player
echo "Now the value of the first_player after unsetting is :: $first_player"

# shell provides a way to mark a variable as read-only varaiables
second_player="Hermoinie"
readonly second_player
second_player="Granger"