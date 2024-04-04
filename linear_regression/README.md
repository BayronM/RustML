# Simple Linear Regression in Rust

Linear regression aims to fit a linear equation to observed data to understand the relationship between two variables.

## Concept

Linear regression analyzes data points by fitting a linear equation to the observed data. The linear equation is:

\[y = mx + b\]

The terms are defined as follows:
- \(y\) is the value we're trying to predict.
- \(x\) is the independent variable we use to make predictions.
- \(m\) is the slope of the line, which represents the effect \(x\) has on \(y\).
- \(b\) is the y-intercept, which is the value of \(y\) when \(x\) is 0.

To find the best-fit line, we need to calculate \(m\) and \(b\), which minimizes the differences between the observed values and the values predicted by our linear equation. The formulas for calculating \(m\) (slope) and \(b\) (intercept) from a dataset are:

\[m = \frac{n(\sum xy) - (\sum x)(\sum y)}{n(\sum x^2) - (\sum x)^2}\]

\[b = \frac{(\sum y) - m(\sum x)}{n}\]

where:
- \(n\) is the number of observations,
- \(\sum x\) is the sum of the independent variable values,
- \(\sum y\) is the sum of the dependent variable values,
- \(\sum xy\) is the sum of the product of each pair of independent and dependent variable values,
- \(\sum x^2\) is the sum of the squares of the independent variable values.


