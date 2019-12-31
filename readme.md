# Maze Generation Project

Maze generation library project to power a future maze generation lambda

General goals:

- rust api to generate mazes up to 100x100 units or larger
- support a variety of maze generation algorithms
- support a variety of 'encodings' (binary, PNG)

Why Rust?

- Mostly because I want to learn it
- Seemed appropriate for an efficient, mostly algorithm driven project dealing with binary data.

# TODO

## Milestone 0: Maze generation module sophisticated enough to support basic maze generation algorithms

- is open check [done]
- ability to print mazes for debugging [done]

## Milestone 1: Sidewinder and Binary Tree algorithm

- implement binary tree algorithm [done]
- support efficient way to iterate over cells in different orders (e.g. NE -> SW)
- generalize binary tree algorithm to take direction paramaters
- implement sidewinder algorithm

## Milestone 2: Unbiased algorithms (Wilson's)

- Implement Wilson's algorithm
- Implement Aldous-Broder algorithm
- Experiment w/Wilsons's-AB hybrid
