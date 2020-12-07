import sys, re, functools

def part_one(luggage_rules):
    @functools.lru_cache(None)
    def eventually_contains(bag, target):
        return any(other == target or eventually_contains(other, target)
                   for quantity, other in luggage_rules[bag])
    return sum(eventually_contains(bag, 'shiny gold') for bag in luggage_rules)

def part_two(luggage_rules):
    @functools.lru_cache(None)
    def count_bags_inside(bag):
        return sum(quantity + quantity*count_bags_inside(other)
                   for quantity, other in luggage_rules[bag])
    return count_bags_inside('shiny gold')

def parse_file(f):
    convert = lambda lst: [(int(a), b) for a, b in lst]
    return {bag:convert(re.findall('(\d+) ([a-zA-Z ]+) bags?', others))
            for bag, others in [line.split(' bags contain ') for line in f]}

if __name__ == '__main__':
    parsed_input = parse_file(sys.stdin)
    print('Part One:', part_one(parsed_input))
    print('Part Two:', part_two(parsed_input))
