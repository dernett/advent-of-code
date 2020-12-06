import sys

def part_one(groups):
    return sum(len(set.union(*map(set, group))) for group in groups)

def part_two(groups):
    return sum(len(set.intersection(*map(set, group))) for group in groups)

def read_input(f):
    return [line.split() for line in f.read().split('\n\n')]

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
