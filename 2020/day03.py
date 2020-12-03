import sys, math, itertools

def count_trees(grid, slope, start=(0, 0)):
    width, (sx,sy), (dx,dy) = len(grid[0]), start, slope
    return sum(row[x % width] == '#' for row, x in
               zip(grid[sy::dy], itertools.count(sx, dx)))

def part_one(grid):
    return count_trees(grid, (3, 1))

def part_two(grid):
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    return math.prod(count_trees(grid, slope) for slope in slopes)

def read_input(f):
    return [line.rstrip('\n') for line in f]

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
