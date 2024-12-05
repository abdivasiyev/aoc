use crate::aoclib;
use std::collections::HashMap;

#[derive(Default)]
pub struct Aoc2024_05 {
    rules: HashMap<i64, Vec<i64>>,
    updates: Vec<Vec<i64>>,
}

impl Aoc2024_05 {
    pub fn new() -> Self {
        Default::default()
    }

    fn is_right_order(&self, update: &Vec<i64>) -> bool {
        let mut indexes = HashMap::new();

        for i in 0..update.len() {
            indexes.insert(update[i], i);
        }

        for i in 0..update.len() {
            if self.rules.contains_key(&update[i]) {
                let pages = &self.rules[&update[i]];

                for page in pages {
                    if let Some(index) = indexes.get(page) {
                        if *index <= i {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn correct_order(&self, update: &Vec<i64>) -> Vec<i64> {
        let mut indexes = HashMap::new();

        for i in 0..update.len() {
            indexes.insert(update[i], i);
        }

        let mut correct_order = update.clone();
        let mut i: usize = 0;
        let mut swapped = false;

        while i < correct_order.len() {
            if !self.rules.contains_key(&correct_order[i]) {
                i+=1;
                continue;
            }

            let pages = &self.rules[&correct_order[i]];

            for page in pages {
                if let Some(index) = indexes.get_mut(page) {
                    if *index >= i {
                        continue
                    }

                    let index = index.clone();

                    indexes.insert(correct_order[index], i);
                    indexes.insert(correct_order[i], index);

                    correct_order.swap(i, index);
                    swapped = true;
                    i = index;
                    break;
                }
            }

            if !swapped {
                i += 1;
            } else {
                swapped = false;
            }
        }

        /*
        97,13,75,29,47
        97,75,13,29,47
        97,75,29,13,47
        97,75,29,47,13
        97,75,47,29,13
        */

        if !self.is_right_order(&correct_order) {
            println!("Error on {:?} -----> {:?}", update, &correct_order);
        }

        correct_order
    }
}

impl aoclib::Solution for Aoc2024_05 {
    fn id(&self) -> (i32, i32) {
        (2024, 5)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024_5.txt");
        // let lines = aoclib::read_lines("test.txt");

        let mut iter = lines.iter();

        while let Some(line) = iter.next() {
            if line == "" {
                break;
            }

            let (x, y) = line.split_once("|").unwrap();
            let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

            if self.rules.contains_key(&x) {
                self.rules.get_mut(&x).unwrap().push(y);
            } else {
                self.rules.insert(x, vec![y]);
            }
        }

        while let Some(line) = iter.next() {
            self.updates
                .push(line.split(",").map(|x| x.parse::<i64>().unwrap()).collect());
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut sum_of_middle_pages = 0;

        for i in 0..self.updates.len() {
            if self.is_right_order(&self.updates[i]) {
                sum_of_middle_pages += self.updates[i][self.updates[i].len() / 2];
            }
        }

        aoclib::output(sum_of_middle_pages)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut sum_of_middle_pages = 0;

        for i in 0..self.updates.len() {
            if self.is_right_order(&self.updates[i]) {
                continue;
            }
            
            let corrected = self.correct_order(&self.updates[i]);
            sum_of_middle_pages += corrected[corrected.len() / 2];
        }

        aoclib::output(sum_of_middle_pages)
    }
}
