import sys, math, itertools

def map_combs(lst, size, func, pred):
    return map(func, filter(pred, itertools.combinations(lst, size)))

def part_one(lst):
    return next(map_combs(lst, 2, math.prod, lambda x: sum(x) == 2020), None)

def part_two(lst):
    return next(map_combs(lst, 3, math.prod, lambda x: sum(x) == 2020), None)

def read_input(f):
    return [int(x) for x in f]

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
