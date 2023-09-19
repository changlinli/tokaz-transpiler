myValue : Int
myValue = 5

identity : (t : Type) -> (x : t) -> t
identity t x = x

anotherCopyOfMyValue : Int
anotherCopyOfMyValue = identity 5
