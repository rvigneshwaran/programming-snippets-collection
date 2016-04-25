#vector Manipulation using operators
vectorInstance1 <- c(36,30,72)
vectorInstance2 <- c(6,6,6)

#addition
print(vectorInstance1 + vectorInstance2)

#Substraction
print(vectorInstance1 - vectorInstance2)

#Modulo Division
print(vectorInstance1 %% vectorInstance2)

#Modulo Division
print(vectorInstance1 %% vectorInstance2)

#Logical operators
print(vectorInstance1 > vectorInstance2)
print(vectorInstance1 < vectorInstance2)
print(vectorInstance1 == vectorInstance2)
print(vectorInstance1 <= vectorInstance2)
print(vectorInstance1 >= vectorInstance2)
print(vectorInstance1 != vectorInstance2)
print(vectorInstance1 && vectorInstance2)
print(vectorInstance1 || vectorInstance2)

#Assignmnet Operators
"Right Instance by Single Arrow" -> rightInstance
print(rightInstance)
"Right Instance by Double Arrow" ->> rightInstance1
print(rightInstance1)
leftInstance <- "Left Instance by Single Arrow"
print(leftInstance)
leftInstance1 <<- "Left Instance by Double Arrow"
print(leftInstance1)
equalOperatorInstance = "Assignment using Equals Operator"
print(equalOperatorInstance)

#Produce Series of Numbers
rangeOfNumberInstance <- 1:10
print(rangeOfNumberInstance)

#Find whether the Element is in Vector
findVectorInstance <- c("India","Cambodia","Singapore")
index1Instance <- "India"
print(index1Instance %in% findVectorInstance)

#Multiple Matric With Their own Transpose
matrixcInstance = matrix( c(2,6,5,1,10,4), nrow = 2,ncol = 3,byrow = TRUE)
t = matrixcInstance %*% t(matrixcInstance)
print(t)