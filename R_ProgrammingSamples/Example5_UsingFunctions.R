#using For Loop and Function to Iterate the Vector values
iterativeList <- function(inputvalues){
  for(index in inputvalues){
    cat(index,"\n")	
  }
}

#using For Loop and Function to Iterate the Vector values
iterativeList <- function(){
  for(index in 1:10){
    cat("Index is at the Value ::  ",index)	
  }
}
myVectorInstance <- c("India","Cambodia","Singapore","Thailand")
iterativeList(myVectorInstance)
iterativeList()

#String Manipulations
string1Instance <- "String 1"
string2Instance <- "String 2"
string3Instance <- "String 3"
print(paste(string1Instance,string2Instance,string3Instance))
print(paste(string1Instance,string2Instance,string3Instance,sep="-"))
cat("Printing Characters in String :: ",nchar(string1Instance))
string4Instance <- toupper(string1Instance)
string5Instance <- toupper(string1Instance)
cat("Upper Case of string1Instance :: ",string4Instance)
cat("lower Case in string1Instance :: ",string5Instance)
string6Instance <- substring(string1Instance,2,4)
cat("Demonstating SubString in string1Instance :: ",string6Instance)