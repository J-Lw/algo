## Intro to Algorithms 

- Informally, an algorithm is a well-defined computational procedure that takes value as input and produces value as output in finite time.
- An algorithm is a sequence of computational steps that transforms an input into an output.
- An algorithm can also be viewed as a tool for solving a well-specified computational problem in that the problem statement specifies the desired input output relationship.
- An algorithm can describe a specific computational procedure for achieving a specific input output relationship for all instances of a given problem.

#### Sorting Problem Formal Def
```
- Input: A sequence of n numbers (a1, a2, ..., an).
- Output: A permutation (a1, a2, ..., an) of the input sequence such that a1 < a2 < ... < an. 
```
- Given the sequence (31, 41, 59, 26, 41, 58), a correct sorting algorithm will return as output the sequence (26, 31, 41, 41, 58, 59).
- Such an input sequence is called an instance of the sorting problem.
- Generally, an instance of a specific problem type consists of the input that satisfies whatever problem statement constraints that is needed to compute a solution to the problem.
- Sorting is a fundamental operation in computer science because many programs use it as a intermediate step.
- There are many good sorting algorithms.
- The correct sorting algorithm to be used depends on things such as the number of items to be sorted, the extent to which items are already sorted, restrictions on item values, computer architecture, and storage device type.

## Algorithmic Correctness
- An algorithm for a computational problem can be considered correct if for every problem instance provided as input it halts and outputs the correct solution for the problem instance.  
