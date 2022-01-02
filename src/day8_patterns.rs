use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Digit(pub u8);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digits(pub [Digit; 10]);

type DigitToNumber = HashMap<Digit, u8>;

#[derive(Debug)]
pub struct PossiblePermutation {
    pub sorted_digits: Digits,
    pub digit_to_number: DigitToNumber,
}

#[derive(Default)]
pub struct PossiblePermutations(HashMap<Digits, DigitToNumber>);

impl PossiblePermutations {
    pub fn find(&self, digits: &Digits) -> DigitToNumber {
        self.0.get(digits).unwrap().clone()
    }

    pub fn decode(&self, digits: &Digits, output: &[Digit; 4]) -> u32 {
        let digit_to_number = self.find(digits);
        let mut out = 0u32;
        for i in 0..4 {
            out = (out * 10) + digit_to_number.get(&output[i]).unwrap().clone() as u32;
        }
        out
    }
}

// Which segment maps to which segment.
type PatternMapping = [usize; 7];

// Numbers:
//
//  0
// 1 2
//  3
// 4 5
//  6
//
// 0: 012 456
// 1:   2  5
// 2: 0 234 6
// 3: 0 23 56
// 4:  123 5
// 5: 01 3 56
// 6: 01 3456
// 7: 0 2  5
// 8: 0123456
// 9: 0123 56

const DIGITS: [u8; 10] = [
    //0123456
    0b1110111, // 0
    0b0010010, // 1
    0b1011101, // 2
    0b1011011, // 3
    0b0111010, // 4
    0b1101011, // 5
    0b1101111, // 6
    0b1010010, // 7
    0b1111111, // 8
    0b1111011, // 9
];

fn create_pattern_mappings_inner(
    out: &mut Vec<PatternMapping>,
    so_far: &mut PatternMapping,
    pos: usize,
    used: &mut [bool; 7],
) {
    if pos >= 7 {
        out.push(so_far.clone());
        return;
    }
    for i in 0..7 {
        if !used[i] {
            used[i] = true;
            so_far[pos] = i;
            create_pattern_mappings_inner(out, so_far, pos + 1, used);
            used[i] = false;
        }
    }
}

fn create_pattern_mappings() -> Vec<PatternMapping> {
    let mut v = Vec::new();
    create_pattern_mappings_inner(&mut v, &mut [0usize; 7], 0, &mut [false; 7]);
    v
}

pub fn create_possible_permutations() -> PossiblePermutations {
    let mut pp = PossiblePermutations::default();
    let mappings = create_pattern_mappings();
    for mapping in mappings {
        let mut tmp = [(Digit::default(), 0u8); 10];
        for digit in 0..10 {
            tmp[digit].1 = digit as u8;
            for segment in 0..7 {
                tmp[digit].0 .0 |= ((DIGITS[digit] >> mapping[segment]) & 1) << segment;
            }
        }
        tmp.sort();
        let sorted_digits = Digits(tmp.map(|(dig, _)| dig));
        let digit_to_number: HashMap<Digit, u8> = tmp.into_iter().collect();
        pp.0.insert(sorted_digits, digit_to_number);
    }
    pp
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_patterns() {
        let v = create_pattern_mappings();
        assert_eq!(v.len(), 5040);
        assert_eq!(v[0], [0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(v[1], [0, 1, 2, 3, 4, 6, 5]);
    }

    #[test]
    fn test_permutations() {
        let v = create_possible_permutations();
        assert_eq!(v.0.len(), 5040);
        dbg!(v.0.iter().next().unwrap());
    }
}
