import sys, re

def part_one(lst):
    return sum(a <= d.count(c) <= b for a,b,c,d in lst)

def part_two(lst):
    return sum((d[a-1] == c) ^ (d[b-1] == c) for a,b,c,d in lst)

def read_input(f):
    pattern = re.compile('(\d+)-(\d+) (\w): (\w+)')
    convert = lambda a, b, c, d: (int(a), int(b), c, d)
    return [convert(*pattern.search(x).groups()) for x in f]

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
