use anyhow::{Result, anyhow};
use chumsky::prelude::*;
use good_lp::{Expression, Solution, SolverModel, constraint, highs, variables};
use util::InputFile;

#[derive(Debug, Clone)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    requirements: Vec<u32>,
}

impl Machine {
    fn new(lights_str: &str, buttons: &[Vec<u32>], requirements: Vec<u32>) -> Self {
        let lights =
            lights_str.char_indices().fold(
                0,
                |mask, (i, c)| if c == '#' { mask | (1 << i) } else { mask },
            );

        let buttons = buttons
            .iter()
            .map(|button| button.iter().fold(0, |mask, i| mask | (1 << i)))
            .collect::<Vec<_>>();

        Self {
            lights,
            buttons,
            requirements,
        }
    }

    fn fewest_presses(&self) -> Option<u32> {
        // Pressing a button twice is the same as doing nothing
        // So, just try every possible subset of buttons and pick
        // the smallest one that gives us the desired state.
        let push = |mask| {
            let mut state = 0;
            for i in 0..self.buttons.len() {
                if (mask & (1 << i)) != 0 {
                    state ^= self.buttons[i];
                }
            }
            state
        };

        (0..2_u64.pow(self.buttons.len().try_into().unwrap_or(u32::MAX)))
            .filter(|&mask| push(mask) == self.lights)
            .map(|mask| mask.count_ones())
            .min()
    }

    fn fewest_presses_joltage(&self) -> Option<u32> {
        variables! {vars: 0 <= x[self.buttons.len()] (integer); }

        let objective = (0..vars.len()).map(|i| x[i]).sum::<Expression>();

        let mut problem = vars.minimise(objective.clone()).using(highs);

        for (i, required_value) in self.requirements.iter().enumerate() {
            let idxs = self
                .buttons
                .iter()
                .enumerate()
                .filter(|&(_, button)| button & (1 << i) != 0)
                .map(|(i, _)| i);

            problem.add_constraint(constraint!(
                idxs.map(|i| x[i]).sum::<Expression>() == *required_value
            ));
        }

        let solution = problem.solve().ok()?;

        Some(solution.eval(objective) as u32)
    }
}

#[derive(Debug)]
struct Input {
    machines: Vec<Machine>,
}

impl Input {
    fn parse(file: &InputFile) -> Result<Self> {
        Self::parser()
            .parse(&file.contents)
            .into_result()
            .map_err(|errs| {
                file.print_diagnostics(errs);
                anyhow!("Failed to parse input file '{}'", file.path.display())
            })
    }

    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<Rich<'src, char>>> {
        let lights = just('[')
            .ignore_then(one_of(".#").repeated().at_least(1).to_slice())
            .then_ignore(just(']'));

        let button = just('(')
            .ignore_then(
                util::unsigned::<u32>(10)
                    .separated_by(just(','))
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(')'));

        let requirements = just('{')
            .ignore_then(
                util::unsigned::<u32>(10)
                    .separated_by(just(','))
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just('}'));

        let line = lights
            .padded()
            .then(button.padded().repeated().at_least(1).collect::<Vec<_>>())
            .then(requirements)
            .map(|((lights_str, buttons), requirements)| {
                Machine::new(lights_str, &buttons, requirements)
            });

        line.separated_by(text::newline())
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(|lines| Self { machines: lines })
    }

    fn part_one(&self) -> u32 {
        self.machines
            .iter()
            .map(|machine| machine.fewest_presses().unwrap())
            .sum()
    }

    fn part_two(&self) -> u32 {
        self.machines
            .iter()
            .map(|machine| machine.fewest_presses_joltage().unwrap())
            .sum()
    }
}

fn main() -> Result<()> {
    let input_file = InputFile::read("event-2025/input/day10.txt")?;
    let input = Input::parse(&input_file)?;

    println!("{}", input.part_one());
    println!("{}", input.part_two());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn fewest_presses_works() {
        let contents = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.machines[0].fewest_presses(), Some(2));
        assert_eq!(example.machines[1].fewest_presses(), Some(3));
        assert_eq!(example.machines[2].fewest_presses(), Some(2));
    }

    #[test]
    fn fewest_presses_joltage_works() {
        let contents = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};
        let example = Input::parse(&contents.into()).unwrap();
        assert_eq!(example.machines[0].fewest_presses_joltage(), Some(10));
        assert_eq!(example.machines[1].fewest_presses_joltage(), Some(12));
        assert_eq!(example.machines[2].fewest_presses_joltage(), Some(11));
    }

    #[test]
    fn example_works() {
        let contents = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};
        let example = Input::parse(&contents.into()).unwrap();
        dbg!(&example);
        assert_eq!(example.part_one(), 7);
        assert_eq!(example.part_two(), 33);
    }
}
