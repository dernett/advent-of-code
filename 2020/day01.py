import sys, math, itertools

def read_input(f):
    return list(map(int, f.readlines()))

def prod_combs(nums, size, pred):
    return map(math.prod, filter(pred, itertools.combinations(nums, size)))

def part_one(nums):
    return next(prod_combs(nums, 2, lambda x: sum(x) == 2020), None)

def part_two(nums):
    return next(prod_combs(nums, 3, lambda x: sum(x) == 2020), None)

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
