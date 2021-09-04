Below is a proposal for SetQL, a query language based on set-builder notation.

# Syntax

### Reserved Symbols

| First-Order Logic | First-Order QL     | Definition                                                                                                                                   |
| ----------------- | ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------- |
| 𝓠                 | /Q                 | The 'query set', ie. the resultant query                                                                                                     |
| ∈                 | /e                 | Preceded by a bounded variable v and followed by a set S, this represents 'v in S'. Can also be used for substring and sub-array conditions. |
| ∑                 | /S                 | Followed by a set, this represents the sum of a set of real numbers                                                                          |
| Π                 | /P                 | Followed by a set, this represents the product of a set of real numbers                                                                      |
| &#124;            | &#124;             | Absolute value for a number or the size of a set                                                                                             |
| ∧                 | ^                  | Conjunction of two statements                                                                                                                |
| ∨                 | v                  | Disjunction of two statements                                                                                                                |
| ⊆                 | /c                 | Subset of a set                                                                                                                              |
| ≤, ≥, ≠, =, >, <  | <=,>=, !=, =, >, < | Represents comparison operators for scalars and strings                                                                                      |
| idx               | idx                | Index of an element in a set                                                                                                                 |

### Fields

Accessing field 'f' of item 'x' should be represented by a function f(x). If f does not exist as a field of x, an implicit field will be created.

# Building Blocks

### Conditional Subset

```
/Q = {v /e S | c}
```

where c is some condition on the item v, and S is a set

### Set Size

```
/Q /c {v /e S} ^ |/Q| = n
```

where n is a number representing the size of Q

### Set Ordering

```
/Q = {v /e S | f(v) >= g(w) ^ idx(v) > idx(w) ^ w /e S}
```

where f, g are fields of items in S, and v, w are elements of S

### Introduction of Fields

```
/Q = {v /e S | f(v) = e}
```

where e is some expression, S is a set and f is a field not defined in items of S

### Introduction of Objects

```
/Q = {v | f(v) = e ^ g(v) = d ^ h(v) = c}
```

where f, g, h are fields, e, d, c are expressions, and Q is the set that will hold the objects

### Expressions on Elements

```
/Q = {e | v /e S}
```

where e is an expression

### Functions on Sets

```
/Q = {v | f(v) = h({e | w /in S})}
```

where g is a field of elements of S and h is a function on the set with elements of the type g, a.k.a. an aggregation

###

# Example Queries

### Query all items by a condition

```
/Q = {v /e S | f(v) = 2}
```

### Query first item by condition

```
/Q /c {v /e S | f(v) = 2} ^ |Q| = 1
```

### Query first 'n' items by a condition

```
/Q /c {v /e S | f(v) = 2} ^ |Q| = n
```

### Aggregation with max accumulator grouped by some field

```
/Q = {v | max_f(v) = max({f(w) | w /e S ^ g(v) = g(w)})}
```

### Aggregation with sum accumulator

```
/Q = {v | sum_f(v) = /S({f(w) | w /e S ^ g(v) = g(w)})}
```

### Aggregation with product accumulator

```
/Q = {v | prod_f(v) = /P({f(w) | w /e S ^ g(v) = g(w)})}
```

### Aggregation with filter then count accumulator

```
/Q = {v | count(v) = |{w /e S | g(v) = g(w) ^ h(w) < 100}|}
```

# Implicit Conditions on Sets

Consider the aggregate expression from above

```
/Q = {v | count(v) = |{w /e S | g(v) = g(w) ^ h(w) < 100}|}
```

Firstly, a brief summary of the expression is as follows: group by `g` elements of `S` that have `h` < 100, and store the number of elements for each grouping of `g` in a field `count`. The output will thus be a table of fields `g` and `count`, which is represented by our query set 𝓠. Mathematically speaking, 𝓠 could be an infinite set with majority elements v giving count(v) = 0, representing all the possible values of g that do not exist in S. However in SetQL 𝓠 should exclude all such fields as they are not useful. This can be represented by an additional condition `g(v) /e {g(w) | w /e S}` on the set being built, meaning there is some row in S that has the same g as rows in 𝓠. In first-order logic, this would be ∀v∈Q ∃w∈S g(v)=g(w). For the purposes of SetQL, however, we will apply this condition implicitly. Thus we see that the specification for the output of 𝓠 may be more specific than the mathematical interpretation of the SetQL expression.

# Executing Queries

The goal for an implementation of SetQL should be to model the set-builder notation expression as a set of operations to perform on every row of the table, thus giving O(n) runtime complexity in a naive table implementation. Sharding and sorted order should also be taken into account to optimize efficiency.
