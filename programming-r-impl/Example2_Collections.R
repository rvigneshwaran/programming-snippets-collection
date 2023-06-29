#create a vector 
vectorInstance <- c("apple","orange","Peaches")
print(vectorInstance)
print(class(vectorInstance))

#creating List With multiple data Types
listInstances <- list(c(1,2,3),tan,"Peaches",TRUE)
print(listInstances)
print(class(listInstances))

#Play With Matrices
matricInstance = matrix(c(1,3,3,3,3,3,3,3,3),nrow=3,ncol=3,byrow=3)
print(matricInstance)

#Array listInstances
arrayInstance = array(c("Green","Yellow"),dim=c(4,4,3))
print(arrayInstance)

#Factor listInstances
vectorInstances <- c("India","Brazil","Argentina","India")
factorInstance <- factor(vectorInstances)
print(nlevels(factorInstance))

#creating a dataframe
dataFrameInstance <- data.frame(
  gender = c("Male","Male","Male"),
  height = c(152,155,165),
  name = c("Name1","Name2","Name3")
)
print(dataFrameInstance)