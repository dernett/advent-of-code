import re
import sys
import functools


def part_one(luggage_rules):
    @functools.cache
    def eventually_contains(bag, target):
        return any(contained_bag == target
                   or eventually_contains(contained_bag, target)
                   for quantity, contained_bag in luggage_rules[bag])
    return sum(eventually_contains(bag, 'shiny gold') for bag in luggage_rules)


def part_two(luggage_rules):
    @functools.cache
    def count_bags_inside(bag):
        return sum(quantity + quantity*count_bags_inside(contained_bag)
                   for quantity, contained_bag in luggage_rules[bag])
    return count_bags_inside('shiny gold')


def parse_file(input_file):
    def parse_contained_bags(contained_bags):
        return [(int(quantity), contained_bag) for quantity, contained_bag
                in re.findall(r'(\d+)\s+([a-z\s]+)\s+bags?', contained_bags)]
    return {bag: parse_contained_bags(contained_bags) for bag, contained_bags
            in [line.split(' bags contain ') for line in input_file]}


if __name__ == '__main__':
    parsed_input = parse_file(open(sys.argv[1]))
    print('Part One:', part_one(parsed_input))
    print('Part Two:', part_two(parsed_input))
