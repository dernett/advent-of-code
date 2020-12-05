import sys, re

is_valid_byr = lambda year: 1920 <= year <= 2002
is_valid_iyr = lambda year: 2010 <= year <= 2020
is_valid_eyr = lambda year: 2020 <= year <= 2030
is_valid_hgt = lambda height, units: 150 <= height <= 193 if units == 'cm' else 59 <= height <= 76

rules = {'byr':('(\d{4})',                     lambda x:    is_valid_byr(int(x))),
         'iyr':('(\d{4})',                     lambda x:    is_valid_iyr(int(x))),
         'eyr':('(\d{4})',                     lambda x:    is_valid_eyr(int(x))),
         'hgt':('(\d+)(cm|in)',                lambda x, y: is_valid_hgt(int(x), y)),
         'hcl':('#[0-9a-f]{6}',                lambda:      True),
         'ecl':('amb|blu|brn|gry|grn|hzl|oth', lambda:      True),
         'pid':('\d{9}',                       lambda:      True),
         'cid':('.*',                          lambda:      True)}

def all_present(passport):
    return all(field in passport for field in rules if field != 'cid')

def all_valid(passport):
    for field, value in passport.items():
        pattern, validator = rules[field]
        match = re.fullmatch(pattern, value)
        if match is None or not validator(*match.groups()):
            return False
    return True

def part_one(passports):
    return sum(all_present(passport) for passport in passports)

def part_two(passports):
    return sum(all_present(passport) and all_valid(passport) for passport in passports)

def read_input(f):
    return [dict(re.findall('(\S+):(\S+)', line)) for line in f.read().split('\n\n')]

if __name__ == '__main__':
    contents = read_input(sys.stdin)
    print('Part One:', part_one(contents))
    print('Part Two:', part_two(contents))
