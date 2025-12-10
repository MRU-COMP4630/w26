# Tutorial 2: Linear algebra and NumPy exercises

Linear algebra is foundational to machine learning, and [NumPy](https://numpy.org/doc/stable/index.html) is a mature Python library that allows you to work efficiently with matrices and vectors.

For this tutorial, I'll be providing a [printed worksheet](linear_alg_exercises.pdf) with some exercises to do **by hand**. You can then check your answers using NumPy. This serves to both refresh your memory on linear algebra as well as gain some familiarity working with NumPy.

## NumPy Basics
By convention, NumPy is imported with the name `np`:

```python
import numpy as np
```

You can then create an N-dimensional array by passing a standard Python `list` to `np.array`:

```python
v = np.array([1, 2]) # a 1-D vector
print(v.shape) # prints (2, )
A = np.array([[1, 2], [3, 4]]) # a 2-D matrix
print(A.shape) # prints (2, 2)
```

The default multiplication operator is **element-wise**. If you want to use matrix multiplication, use the `@` operator, or `dot` function for vectors:

```python
print(A * v)
print(A @ v)
print(v.dot(v)) # or np.dot(v, v)
```

Output:
```
[[1 4]
 [3 8]]

[5, 11]

5
```

Transposing a matrix is quite simple, but the inverse needs the `linalg` submodule:

```python
print(A.T) # Transpose
print(np.linalg.inv(A)) # Matrix inverse
```

Output:
```
[[1 3]
 [2 4]]

[[-2.   1. ]
 [ 1.5 -0.5]]
```

Similarly, `linalg` has useful functions like `norm`, `det`, `solve`... getting familiar with the [docs](https://numpy.org/doc/stable/reference/routines.linalg.html) can be handy!

## More resources
- [Hands-On Machine Learning book: Linear Algebra Refresher](https://github.com/ageron/handson-mlp/blob/main/math_linear_algebra.ipynb)
- [NumPy docs](https://numpy.org/doc/stable/user/index.html)
- [NumPy quickstart](https://numpy.org/doc/stable/user/quickstart.html)