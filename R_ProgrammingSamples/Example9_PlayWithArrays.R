# Create two vectors of different lengths.
inputVector1 <- c(5,9,3)
inputVector2 <- c(10,11,12,13,14,15)

# Take these vectors as input to the array.
vectorsAsArray <- array(c(inputVector1,inputVector2),dim = c(3,3,2))
print(vectorsAsArray)