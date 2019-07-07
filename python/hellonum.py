import numpy as np

a = [1,2,3,4,5]
x = np.array(a)

print("type of x is ", type(x), " of element ", x.dtype)

y = np.array([4, 3, 2, 1, 0])
# can deal with slices like normal arrays
y[2:4] = 1, 2
print("product of ", x, " and ", y, " is ", x*y)
print("the dot product is ", np.dot(x, y))

x = [[3, 4], [2, 3]]
a = np.array(x)
print("shape is ", a.shape)
b = np.array([[4, 2],[0, 2]])

print("here is another 2-dimensional vector: ", b)
print("their product is ", a*b)

