mod utils;

use lazy_static::lazy_static;
use ::regex::Regex;
use ::regex;

lazy_static! {
    static ref NAME_RE: Regex = regex!(r"^([a-zA-Z]+)\s");
    static ref MAGIMIN_RE: Regex = regex!(r"([a-e])(\d+)");
    static ref SENSE_RE: Regex = regex!(r"([+|-])(taste|feel|sight|smell|sound)");
    static ref PRICE_RE: Regex = regex!(r"\s(\d+)$");
}

#[derive(Debug, Eq, Ord)]
pub struct Ingredient {
    name: String,

    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,

    mutamin: usize,

    taste: isize,
    feel: isize,
    sight: isize,
    smell: isize,
    sound: isize,

    price: usize,
}

impl PartialEq for Ingredient {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mutamin == other.mutamin
    }
}

impl PartialOrd for Ingredient {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp = self.sense_score().partial_cmp(&other.sense_score());
        if cmp == Some(std::cmp::Ordering::Equal) {
            self.mutamin.partial_cmp(&other.mutamin)
        } else {
            cmp
        }
    }
}

impl Ingredient {
    pub fn sense_score(&self) -> isize {
        self.taste + self.feel + self.sight + self.smell + self.sound
    }

    pub fn load(filename: &str) -> Vec<Ingredient> {
        utils::get_input(filename).filter(|line| !line.starts_with("#") && line.len() > 1)
        .map(|line| {
            let mut a = 0;
            let mut b = 0;
            let mut c = 0;
            let mut d = 0;
            let mut e = 0;
            let mut taste = 0;
            let mut feel = 0;
            let mut sight = 0;
            let mut smell = 0;
            let mut sound = 0;
    
                for captures in MAGIMIN_RE.captures_iter(&line) {
                    let m = &captures[1];
                    let value = captures[2].parse::<usize>().unwrap();
                    match m {
                        "a" => {a = value}
                        "b" => {b = value}
                        "c" => {c = value}
                        "d" => {d = value}
                        "e" => {e = value}
                        _ => panic!()
                    };
                }

                for captures in SENSE_RE.captures_iter(&line) {
                    let value = &captures[1];
                    let sense = &captures[2];
                    let sense_value = match value {
                        "+" => { 1 }
                        "-" => { -1 }
                        _ => panic!()
                    };
                    match sense {
                        "taste" => {taste = sense_value}
                        "feel" => {feel = sense_value}
                        "sight" => {sight = sense_value}
                        "smell" => {smell = sense_value}
                        "sound" => {sound = sense_value}
                        _ => panic!()
                    };
                }

            Ingredient {
                    name: NAME_RE.captures_iter(&line).next().unwrap()[1].to_owned(),
                    a,
                    b,
                    c,
                    d,
                    e,
                    mutamin: a+b+c+d+e,
                    taste,
                    feel,
                    sight,
                    smell,
                    sound,
                    price: PRICE_RE.captures_iter(&line).next().unwrap()[1].parse::<usize>().unwrap(),
                }
        }).collect()
    }
}

#[derive(Default, Debug, Clone, Eq, Ord)]
pub struct IngredientRatio {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    taste: isize,
    feel: isize,
    sight: isize,
    smell: isize,
    sound: isize,
    count: usize,
    min: usize,
    max: usize,
    price: usize,
}

impl PartialEq for IngredientRatio {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.max == other.max
    }
}

impl PartialOrd for IngredientRatio {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut cmp = self.sense_score().partial_cmp(&other.sense_score());
        if cmp == Some(std::cmp::Ordering::Equal) {
            cmp = self.max.partial_cmp(&other.max);
        }
        if cmp == Some(std::cmp::Ordering::Equal) {
            cmp = other.price.partial_cmp(&self.price);
        }
        cmp
    }
}

impl IngredientRatio {
    fn clamp_sense(s: isize) -> isize {
        if s < 0 {
            -1
        } else if s > 0 {
            1
        } else {
            0
        }
    }

    pub fn sense_score(&self) -> isize {
        IngredientRatio::clamp_sense(self.taste) + IngredientRatio::clamp_sense(self.feel) + IngredientRatio::clamp_sense(self.sight) + IngredientRatio::clamp_sense(self.smell) + IngredientRatio::clamp_sense(self.sound)
    }

    pub fn senses_satisfied(&self, t: &IngredientRatio) -> bool {
        let sm = [self.taste, self.sight, self.feel, self.smell, self.sound];
        let tm = [t.taste, t.sight, t.feel, t.smell, t.sound];
        for i in 0..sm.len() {
            if sm[i] <= 0 && tm[i] > 0 {
                return false;
            }
        }
        true
    }

    fn is_possible_ingredient(&self, i: &Ingredient) -> bool {
        let tm = [self.a, self.b, self.c, self.d, self.e];
        let im = [i.a, i.b, i.c, i.d, i.e];
        for i in 0..tm.len() {
            if im[i] > 0 && tm[i] == 0 {
                return false
            }
        }
        true
    }

    fn satisfying_ratio(&self, target: &IngredientRatio) -> Option<usize> {
        if self.count == 0 { return Some(0); }
        let sm = [self.a, self.b, self.c, self.d, self.e];
        let tm = [target.a, target.b, target.c, target.d, target.e];
        let mut target_ratio = 0;
        let mut mismatch = false;
        for i in 0..sm.len() {
            if sm[i] == 0 && tm[i] == 0 {
                continue;
            }

            if sm[i] != 0 && tm[i] == 0 {
                return None;
            }

            if mismatch { continue; }

            mismatch |= sm[i] == 0 && tm[i] != 0 || sm[i] % tm[i] > 0;

            if target_ratio == 0 {
                target_ratio = sm[i] / tm[i];
            } else {
                mismatch |= sm[i] / tm[i] != target_ratio;
            }
        }

        Some(if mismatch { 0 } else { target_ratio })
    }
}

pub fn solve<'a>(ingredient_pool: &'a[Ingredient], candidate_recipe: &mut Vec<&'a str>, candidate_ratio: &IngredientRatio, target_ratio: &IngredientRatio, acc: &mut Vec<(Vec<&'a str>, IngredientRatio)>) {
    if (target_ratio.price > 0 && candidate_ratio.price > target_ratio.price) || candidate_ratio.count > target_ratio.count || candidate_ratio.max > target_ratio.max {
        return;
    }

    let curr_scale = match candidate_ratio.satisfying_ratio(target_ratio) {
        None => { return; }
        Some(s) => s
    };

    if ingredient_pool.len() == 0 {
        if curr_scale > 0 && candidate_ratio.max >= target_ratio.min && candidate_ratio.senses_satisfied(target_ratio) {
            print("++ ", candidate_ratio.count, candidate_ratio.max, candidate_ratio.sense_score(), candidate_ratio.price, &candidate_recipe);
            acc.push((candidate_recipe.to_owned(), candidate_ratio.clone()));
        }
        return;
    }

    // either we choose to ignore the current ingredient...
    solve(&ingredient_pool[1..ingredient_pool.len()], candidate_recipe, candidate_ratio, target_ratio, acc);

    // ...or we try adding the current ingredient, with the potential to add more copies of it
    let i = &ingredient_pool[0];
    let new_ratio = IngredientRatio {
        a: candidate_ratio.a + i.a,
        b: candidate_ratio.b + i.b,
        c: candidate_ratio.c + i.c,
        d: candidate_ratio.d + i.d,
        e: candidate_ratio.e + i.e,
        taste: candidate_ratio.taste + i.taste,
        feel: candidate_ratio.feel + i.feel,
        sight: candidate_ratio.sight + i.sight,
        smell: candidate_ratio.smell + i.smell,
        sound: candidate_ratio.sound + i.sound,
        count: candidate_ratio.count + 1,
        min: candidate_ratio.min,
        max: candidate_ratio.max + i.mutamin,
        price: candidate_ratio.price + i.price,
    };
    candidate_recipe.push(&i.name);
    solve(ingredient_pool, candidate_recipe, &new_ratio, target_ratio, acc);
    candidate_recipe.pop();
}

pub fn print(prefix: &str, count: usize, magimins: usize, sense: isize, price: usize, ingredients: &Vec<&str>) {
    let mut c = 1;
    let mut curr_name = &ingredients[0];
    let mut compact_names = Vec::new();
    for name in ingredients.iter() {
        if name == curr_name {
            c += 1;
        } else {
            compact_names.push(format!("{}x {}", c, curr_name));
            curr_name = name;
            c = 1;
        }
    }
    compact_names.push(format!("{}x {}", c, curr_name));

    println!("{}{} ingredients, {} magimins, {} sense score, ${}\n\t{}", prefix, count, magimins, sense, price, compact_names.join("\n\t"));
}

pub fn main() {
    let mut acc = Vec::new();
    let mut candidate_recipe = Vec::new();
    let target = IngredientRatio{
        a: 2,
        b: 0,
        c: 1,
        d: 1,
        e: 0,

        taste: 0,
        feel: 0,
        sight: 0,
        smell: 0,
        sound: 0,
    
        count: 10,
        min: 480,
        max: 575,
        price: 0,
    };

    let mut ingredients = Ingredient::load("ingredients");
    let old_len = ingredients.len();
    ingredients.retain(|i| target.is_possible_ingredient(i) );
    ingredients.sort();

    println!("Found {} ingredients, only {} are candidates", old_len, ingredients.len());

    solve(&ingredients.as_slice(), &mut candidate_recipe, &IngredientRatio::default(), &target, &mut acc);
    acc.sort_by_key(|(_, ratio)| ratio.clone() );
    for (names, ratio) in acc.into_iter() {
        print("", ratio.count, ratio.max, ratio.sense_score(), ratio.price, &names);
    }
}