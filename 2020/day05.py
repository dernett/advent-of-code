import sys

def generate_seat_id(boarding_pass):
    return int(boarding_pass.translate(str.maketrans('BFRL', '1010')), 2)

def part_one(boarding_passes):
    return max(map(generate_seat_id, boarding_passes))

def part_two(boarding_passes):
    taken_seats = set(map(generate_seat_id, boarding_passes))
    all_seats = set(range(min(taken_seats), max(taken_seats) + 1))
    return next(iter(all_seats - taken_seats), None)

def read_input(f):
    return [line.rstrip('\n') for line in f]

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
