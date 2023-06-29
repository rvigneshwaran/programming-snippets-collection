vectorManipulation <- function(inputValues){
  cat("The Elements of the vector are :: ",inputVector)
  #Sequnce of Numbers
  vectorInstance1 <- 1:10
  cat("The Elements of the vector vectorInstance1 :: ",vectorInstance1)
  vectorInstance2 <- 1.5:10.5
  cat("The Elements of the vector vectorInstance2 :: ",vectorInstance2)
  # If the final element specified does not belong to the sequence then it is discarded.
  vectorInstance3 <- 1.2:10.5
  cat("The Elements of the vector vectorInstance3 :: ",vectorInstance3)
  cat("Creating the Sequnce by Incrementing  by 1 value from 2 to 10 :: ",seq(2,10,by = 1))
}

accessElementsByIndex <- function(inputVector,indexByVector){
  valueByIndex <- inputVector[c(indexByVector)]
  print(valueByIndex)
}

manipulationOfVectors <- function(inputVector1,inputVector2){
  cat("The Addition of the two Equal vector is :: ",inputVector1+inputVector2,"\n")
  cat("The Subtraction of the two Equal vector is :: ",inputVector1-inputVector2,"\n")
  cat("The Multiplication of the Equal two vector is :: ",inputVector1*inputVector2,"\n")
  cat("The Division of the two Equal vector is :: ",inputVector1/inputVector2,"\n")
}

inputVector <- c(120,234,78,56,23,90,78,12,1,45,89)
accessElementsByIndex(inputVector,2)
vectorManipulation(inputVector)

#Maniplulation of Equal vectors
inputVector1 <- c(120,234,78,56,23,90,78,12,1,45,89)
inputVector2 <- c(12,23,7,5,2,9,8,1,11,45,89)
manipulationOfVectors(inputVector1,inputVector2)

#Maniplulation of Un Equal vectors
inputVector3 <- c(120,234,78)
inputVector4 <- c(12,23,7,5,2,9)
cat("The Addition of the two Un Equal vector is :: ",inputVector4+inputVector3,"\n")
cat("Sorting the Elements in vector :: ",sort(inputVector1),"\n")
cat("Sorting the Elements in vector :: ",sort(inputVector1,decreasing=TRUE))