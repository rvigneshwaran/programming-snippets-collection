#variable Assignment leftward & Rightward
var.1 <- "leftwardAssignment"
print(var.1)
"RightwardAssignment" -> var.2
print(var.2)

#concatinating Objects using CAT
cat("Operation Using Cat Function ::  var.1 and var.2",var.1,var.2)
cat("Operation Using Cat Function with New Line Character :: ",var.1,var.2,"\n")
cat("To demonstrate the New Line Character in the previous Line")

#finding the class of the data Type
myStringVaribale <- "myStringObject"
print(class(myStringVaribale))

#To know all the variables currently available in the workspace
print(ls())

# List the variables starting with the pattern "var".
print(ls(pattern = "var"))

#The varibales which are starting with . are hidden, So we reveal it
print(ls(all.name = TRUE))