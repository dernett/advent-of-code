import sys

def run_code(instructions):
    accumulator, ip, seen_ips = 0, 0, set()
    while 0 <= ip < len(instructions) and ip not in seen_ips:
        seen_ips.add(ip)
        operation, argument = instructions[ip]
        if operation == 'acc':
            accumulator += argument
        elif operation == 'jmp':
            ip += argument - 1
        ip += 1
    return (accumulator, ip not in seen_ips)

def part_one(instructions):
    return run_code(instructions)[0]

def part_two(instructions):
    def run_with_changes(changes):
        for i, (operation, argument) in enumerate(instructions):
            if operation in changes:
                instructions[i] = (changes[operation], argument)
                yield run_code(instructions)
                instructions[i] = (operation, argument)
    return next(iter(accumulator for accumulator, terminates in
                     run_with_changes({'nop':'jmp', 'jmp':'nop'}) if terminates), None)

def parse_file(f):
    return [(operation, int(argument)) for operation, argument in map(str.split, f)]

if __name__ == '__main__':
    parsed_input = parse_file(sys.stdin)
    print('Part One:', part_one(parsed_input))
    print('Part Two:', part_two(parsed_input))
