#manipulation of Lists
inputList <- c(TRUE,"String1","String2",123,78.65,c("India","Cambodia","Singapore"),"String3")
print(inputList)

names(inputList) = c("India","Cambodia","Singapore")
print(inputList)

accessElementsInList <- function(inputList){
  for(index in inputList){
  	cat("The Index is now Present in the value :: ",index,"\n")
  }
}

accessIndexByValueList <- function(inputList,index){
  cat("The Index is at :: ",inputList[index],"\n")
}

removeIndexByValueList <- function(inputList,index){
  inputList[index] <- NULL
}

mergeInputList <- function(inputList1,inputList2){
  cat("The Merged :ist is :: ",c(inputList1,inputList2),"\n")
}

(listdata,listdata1) <- c(TRUE,"String1","String2",123,78.65,c("India","Cambodia","Singapore"),"String3")
listdata1 <- c(TRUE,"String2","String3",123,78.65,c("Indiana","Canada","Columbia"),"String4")
accessElementsInList(listdata)
accessIndexByValueList(listdata,4)
removeIndexByValueList(listdata,4)
mergeInputList(listdata,listdata1)