
printMatrixByValues <- function(dataValues,nRowvalues,byRow,rowNames,colNames){
  M <- matrix(dataValues, nrow = nRowvalues, byrow = byRow,dimnames = list(rowNames,colNames))
  print(M)
}

accessElementsInMatrix <- function(inputMatrix,col,row){
  print(inputMatrix[col,row])
}

#Create a Proper vector with Correct values
inputDataValue1 <- c(1:9)
printMatrixByValues(inputDataValue1,3,TRUE)

#uNeven values with the range
inputDataValue2 <- c(1:14)
printMatrixByValues(inputDataValue2,3,TRUE)

#print With dimention Names
inputDataValue3 <- c(1:9)
rowNames <- c("Row1","Row2","Row3")
colNames <- c("Col1","Col2","Col3")
printMatrixByValues(inputDataValue3,3,TRUE,rowNames,colNames)
inputMatrix1 <- matrix(c(1:9), nrow =3, byrow = TRUE)
accessElementsInMatrix(inputMatrix1,2,3)