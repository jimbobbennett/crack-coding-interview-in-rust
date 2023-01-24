# Crack code interview problems in Rust 🦀

This repo contains the problems solved during the Crack code interview problems in Rust series from the Microsoft Reactor.

You can find details for the upcoming episodes, and register on the [Microsoft Reactor site](https://developer.microsoft.com/reactor/series/S-1063/).

## Episodes

### Episode 1 - number of islands

You have a map based off a grid of size X by Y. Each grid cell is either marked as land or sea.
Assuming all around the grid is sea, your job is to calculate the number of islands on the map.

An island is defined as a continuous set of grid cells of land that connect either above, below, left or right with another land cell. Diagonal connections are not allowed.

You can catch the recording by selecting the image below

[![Recording](https://img.youtube.com/vi/ugz1YgoZmzI/0.jpg)](https://youtube.com/watch?v=ugz1YgoZmzI)

#### Example

Here is a grid of 6x12:

![A 6 by 12 grid of blue sea with 6 green islands](./number-of-islands/islands-1.png)

The blue cells are sea, the green cells are land. This map has 6 islands:

![A 6 by 12 grid of blue sea with 6 green islands numberd 1 to 6](./number-of-islands/islands-2.png)

#### Code

You can find the final code for this in the [`number-of-islands`](./number-of-islands/) folder.

### Episode 2 - Conway's game of life

[The Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life), also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970.

The board is made up of an `m x n` grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules:

* Any live cell with fewer than two live neighbors dies as if caused by under-population.
* Any live cell with two or three live neighbors lives on to the next generation.
* Any live cell with more than three live neighbors dies, as if by over-population.
* Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the `m x n` grid board, return the next state.

In this problem, given an initial state compute the next state.

#### Example

The state on the left gives the one on the right as the next state.

```output
0 1 0    0 0 0
0 0 1 -> 1 0 1
1 1 1    0 1 1
0 0 0    0 1 0
```

```output
1 1 -> 1 1
1 0    1 1
```

