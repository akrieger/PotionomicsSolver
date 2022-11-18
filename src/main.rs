mod utils;

use ::regex;
use ::regex::Regex;
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use std::array;
use std::fmt;
use std::ops;
use std::sync::atomic;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref NAME_RE: Regex = regex!(r"(^|\s)([a-zA-Z]+)\s");
    static ref MAGIMIN_RE: Regex = regex!(r"([a-e])(\d+)");
    static ref SENSE_RE: Regex = regex!(r"([+|-])(taste|feel|sight|smell|sound)");
    static ref PRICE_RE: Regex = regex!(r"\$(\d+)");
    static ref NUM_AVAILABLE_RE: Regex = regex!(r"x(\d+)");
}

#[derive(Default, Clone, Debug, Eq, Ord)]
pub struct Magimins {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
}

impl Magimins {
    pub fn total(&self) -> usize {
        self.a + self.b + self.c + self.d + self.e
    }

    pub fn new(a: usize, b: usize, c: usize, d: usize, e: usize) -> Magimins {
        Magimins { a, b, c, d, e }
    }

    pub fn as_array(&self) -> [usize; 5] {
        [self.a, self.b, self.c, self.d, self.e]
    }

    pub fn from_array(magimins: [usize; 5]) -> Magimins {
        Magimins::new(
            magimins[0],
            magimins[1],
            magimins[2],
            magimins[3],
            magimins[4],
        )
    }
}

impl fmt::Display for Magimins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a:{}, b:{}, c:{}, d:{}, e:{}",
            self.a, self.b, self.c, self.d, self.e
        )
    }
}

impl ops::Add<&Magimins> for &Magimins {
    type Output = Magimins;

    fn add(self, rhs: &Magimins) -> Magimins {
        Magimins::new(
            self.a + rhs.a,
            self.b + rhs.b,
            self.c + rhs.c,
            self.d + rhs.d,
            self.e + rhs.e,
        )
    }
}

impl ops::Div<&Magimins> for usize {
    type Output = Magimins;

    fn div(self, rhs: &Magimins) -> Magimins {
        let total = rhs.total();
        if total == 0 {
            return Magimins::new(0, 0, 0, 0, 0);
        }
        return Magimins::new(
            self * rhs.a / total,
            self * rhs.b / total,
            self * rhs.c / total,
            self * rhs.d / total,
            self * rhs.e / total,
        );
    }
}

impl PartialEq for Magimins {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a
            && self.b == other.b
            && self.c == other.c
            && self.d == other.d
            && self.e == other.e
    }
}

impl PartialOrd for Magimins {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total().partial_cmp(&other.total())
    }
}

#[derive(Debug, Eq, Ord, Clone)]
pub struct Ingredient {
    name: String,

    magimins: Magimins,

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
        self.name == other.name
    }
}

impl PartialOrd for Ingredient {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Inverse sort.
        let cmp = other.mutamin.partial_cmp(&self.mutamin);
        if cmp == Some(std::cmp::Ordering::Equal) {
            other.sense_score().partial_cmp(&self.sense_score())
        } else {
            cmp
        }
    }
}

impl Ingredient {
    pub fn sense_score(&self) -> isize {
        self.taste + self.feel + self.sight + self.smell + self.sound
    }

    pub fn load(filename: &str) -> Vec<(Ingredient, Option<usize>)> {
        utils::get_input(filename)
            .filter(|line| !line.starts_with("#") && !line.starts_with("//") && line.len() > 1)
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
                        "a" => a = value,
                        "b" => b = value,
                        "c" => c = value,
                        "d" => d = value,
                        "e" => e = value,
                        _ => panic!(),
                    };
                }

                for captures in SENSE_RE.captures_iter(&line) {
                    let value = &captures[1];
                    let sense = &captures[2];
                    let sense_value = match value {
                        "+" => 1,
                        "-" => -1,
                        _ => panic!(),
                    };
                    match sense {
                        "taste" => taste = sense_value,
                        "feel" => feel = sense_value,
                        "sight" => sight = sense_value,
                        "smell" => smell = sense_value,
                        "sound" => sound = sense_value,
                        _ => panic!(),
                    };
                }

                let mut num_available = None;
                for captures in NUM_AVAILABLE_RE.captures_iter(&line) {
                    num_available = Some(captures[1].parse::<usize>().unwrap());
                }

                (
                    Ingredient {
                        name: NAME_RE.captures_iter(&line).next().unwrap()[2].to_owned(),
                        magimins: Magimins::new(a, b, c, d, e),
                        mutamin: a + b + c + d + e,
                        taste,
                        feel,
                        sight,
                        smell,
                        sound,
                        price: PRICE_RE.captures_iter(&line).next().unwrap()[1]
                            .parse::<usize>()
                            .unwrap(),
                    },
                    num_available,
                )
            })
            .collect()
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PotionAttributes {
    magimins: Magimins,
    taste: isize,
    feel: isize,
    sight: isize,
    smell: isize,
    sound: isize,
}

impl PotionAttributes {
    fn satisfying_ratio(&self, target: &IngredientRatio) -> Option<usize> {
        if self.magimins.total() == 0 {
            return Some(0);
        }
        let sm = self.magimins.as_array();
        let tm = target.magimins.as_array();
        let mut target_ratio = 0;
        let mut mismatch = false;
        for i in 0..sm.len() {
            if sm[i] == 0 && tm[i] == 0 {
                continue;
            }

            if sm[i] != 0 && tm[i] == 0 {
                return None;
            }

            if mismatch {
                continue;
            }

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

impl ops::Add<&Ingredient> for &PotionAttributes {
    type Output = PotionAttributes;

    fn add(self, rhs: &Ingredient) -> PotionAttributes {
        PotionAttributes {
            magimins: &self.magimins + &rhs.magimins,
            taste: self.taste + rhs.taste,
            feel: self.feel + rhs.feel,
            sight: self.sight + rhs.sight,
            smell: self.smell + rhs.smell,
            sound: self.sound + rhs.sound,
        }
    }
}

impl PotionAttributes {
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
        PotionAttributes::clamp_sense(self.taste)
            + PotionAttributes::clamp_sense(self.feel)
            + PotionAttributes::clamp_sense(self.sight)
            + PotionAttributes::clamp_sense(self.smell)
            + PotionAttributes::clamp_sense(self.sound)
    }

    pub fn tier(&self) -> &'static str {
        let total = self.magimins.total();
        if total < 60 {
            return "Minor";
        }
        if total < 150 {
            return "Common";
        }
        if total < 290 {
            return "Greater";
        }
        if total < 470 {
            return "Grand";
        }
        if total < 720 {
            return "Superior";
        }
        return "Masterwork";
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct PotionRecipe {
    attributes: PotionAttributes,
    ingredients: Vec<Ingredient>,
    cost: usize,
}

/**
 * Enumerates all recipes using at least one of the first ingredient
 * of a given ingredient pool for the all the potential inputs,
 * up to some maximum number of ingredients,
 * ideally targeting a specific ratio,
 * with a callback indicating whether to early abort a given recipe,
 * and a callback to accept a recipe that passes validation.
 */
pub fn enumerate<'a, 'b, RecipeCb>(
    ingredient_pool: &'a [(Ingredient, Option<usize>)],
    max_ingredients: usize,
    // Recipe so far. Must not change any existing values. Can append new values.
    current_ingredients: &mut Vec<&'b Ingredient>,
    mut current_ratio: PotionAttributes,
    cb: &mut RecipeCb,
) where
    'a: 'b,
    RecipeCb: FnMut(&[&'b Ingredient], &PotionAttributes) -> bool,
{
    if ingredient_pool.len() == 0 {
        return;
    }
    let mandatory_ingredient = &ingredient_pool[0];
    let max_current_ingredient = (max_ingredients - current_ingredients.len())
        .min(mandatory_ingredient.1.unwrap_or(max_ingredients));
    assert!(max_current_ingredient > 0);
    for _ in 1..=max_current_ingredient {
        current_ingredients.push(&mandatory_ingredient.0);
        current_ratio = &current_ratio + &mandatory_ingredient.0;
        if !cb(current_ingredients.as_slice(), &current_ratio) {
            return;
        }
        if current_ingredients.len() >= max_ingredients {
            return;
        }

        let current_ingredients_len = current_ingredients.len();
        for j in 0..ingredient_pool.len() {
            // Recurse starting from the smallest ingredients to skip coarse prefixes.
            enumerate(
                &ingredient_pool[ingredient_pool.len() - j..],
                max_ingredients,
                current_ingredients,
                current_ratio.clone(),
                cb,
            );
            current_ingredients.truncate(current_ingredients_len);
        }
    }
}

#[derive(Default, Debug, Clone, Eq, Ord)]
pub struct IngredientRatio {
    magimins: Magimins,
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
        let mut cmp = self.max.partial_cmp(&other.max);
        if cmp == Some(std::cmp::Ordering::Equal) {
            cmp = self.sense_score().partial_cmp(&other.sense_score());
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
        IngredientRatio::clamp_sense(self.taste)
            + IngredientRatio::clamp_sense(self.feel)
            + IngredientRatio::clamp_sense(self.sight)
            + IngredientRatio::clamp_sense(self.smell)
            + IngredientRatio::clamp_sense(self.sound)
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
        let tm = self.magimins.as_array();
        let im = i.magimins.as_array();
        for i in 0..tm.len() {
            if im[i] > 0 && tm[i] == 0 {
                return false;
            }
        }
        true
    }

    fn satisfying_ratio(&self, target: &IngredientRatio) -> Option<usize> {
        if self.count == 0 {
            return Some(0);
        }
        let sm = self.magimins.as_array();
        let tm = target.magimins.as_array();
        let mut target_ratio = 0;
        let mut mismatch = false;
        for i in 0..sm.len() {
            if sm[i] == 0 && tm[i] == 0 {
                continue;
            }

            if sm[i] != 0 && tm[i] == 0 {
                return None;
            }

            if mismatch {
                continue;
            }

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

pub fn rms(expected: [usize; 5], expected_total: usize, actual: [usize; 5]) -> f64 {
    let mut sum_squares: f64 = 0.0;
    let mut actual_total: usize = 0;
    for i in 0..expected.len() {
        let diff = expected[i].abs_diff(actual[i]) as u32;
        if diff > 0 {
            sum_squares += diff.pow(2) as f64;
        }
        actual_total += actual[i];
    }
    if expected_total != actual_total {
        sum_squares += (expected_total.abs_diff(actual_total)).pow(2) as f64;
    }
    return sum_squares.sqrt();
}

pub fn solve<'a, F>(
    ingredient_pool: &'a [(Ingredient, Option<usize>)],
    mut num_available: Option<usize>,
    candidate_recipe: &mut Vec<&'a str>,
    candidate_ratio: &IngredientRatio,
    target_ratio: &IngredientRatio,
    cb: &mut F,
) where
    F: FnMut(Vec<&'a str>, IngredientRatio, IngredientRatio),
{
    if (target_ratio.price > 0 && candidate_ratio.price > target_ratio.price)
        || candidate_ratio.count > target_ratio.count
        || candidate_ratio.max > target_ratio.max
    {
        return;
    }

    if ingredient_pool.len() == 0 {
        if candidate_ratio.max >= target_ratio.min && candidate_ratio.senses_satisfied(target_ratio)
        {
            cb(
                candidate_recipe.to_owned(),
                candidate_ratio.clone(),
                target_ratio.clone(),
            );
        }
        return;
    }

    // either we choose to ignore the current ingredient...
    solve(
        &ingredient_pool[1..ingredient_pool.len()],
        None,
        candidate_recipe,
        candidate_ratio,
        target_ratio,
        cb,
    );

    // ...or we try adding the current ingredient, with the potential to add more copies of it
    let (i, c) = &ingredient_pool[0];
    if num_available.is_none() {
        num_available = *c;
    }
    num_available = match num_available {
        Some(0) => {
            return;
        }
        Some(x) => Some(x - 1),
        None => None,
    };
    let new_ratio = IngredientRatio {
        magimins: &candidate_ratio.magimins + &i.magimins,
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
    solve(
        ingredient_pool,
        num_available,
        candidate_recipe,
        &new_ratio,
        target_ratio,
        cb,
    );
    candidate_recipe.pop();
}

pub fn print(
    prefix: &str,
    count: usize,
    magimins: usize,
    sense: isize,
    price: usize,
    ingredients: &[&Ingredient],
) {
    let mut c = 0;
    let mut curr_name = &ingredients[0];
    let mut compact_names = Vec::new();
    for name in ingredients.iter() {
        if name == curr_name {
            c += 1;
        } else {
            compact_names.push(format!("{}x {}", c, curr_name.name));
            curr_name = name;
            c = 1;
        }
    }
    compact_names.push(format!("{}x {}", c, curr_name.name));

    println!(
        "{}{} ingredients, {} magimins, {} sense score, ${}\n\t{}",
        prefix,
        count,
        magimins,
        sense,
        price,
        compact_names.join("\n\t")
    );
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum SolveAlgorithm {
    EXACT,
    APPROXIMATE,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Recipe {
    HEALTH,
    MANA,
    STAMINA,
    SPEED,
    FIRE,
    ICE,
    LIGHTNING,
    SHADOW,
    ALERT,
    SIGHT,
    INSIGHT,
    DOWSING,
    POISON,
    DROWSY,
    PETRI,
    SILENCE,
}

impl Recipe {
    pub fn to_magimins(&self) -> Magimins {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        let mut e = 0;

        match self {
            Recipe::HEALTH => {
                a = 1;
                b = 1;
            }
            Recipe::MANA => {
                b = 1;
                c = 1;
            }
            Recipe::STAMINA => {
                a = 1;
                e = 1;
            }
            Recipe::SPEED => {
                c = 1;
                d = 1;
            }
            Recipe::FIRE => {
                a = 1;
                c = 1;
            }
            Recipe::ICE => {
                a = 1;
                d = 1;
            }
            Recipe::LIGHTNING => {
                b = 1;
                d = 1;
            }
            Recipe::SHADOW => {
                b = 1;
                e = 1;
            }
            Recipe::ALERT => {
                b = 3;
                c = 4;
                d = 3;
            }
            Recipe::SIGHT => {
                a = 3;
                b = 4;
                c = 3;
            }
            Recipe::INSIGHT => {
                a = 4;
                b = 3;
                e = 3;
            }
            Recipe::DOWSING => {
                a = 3;
                d = 3;
                e = 4;
            }
            Recipe::POISON => {
                a = 2;
                c = 1;
                d = 1;
            }
            Recipe::DROWSY => {
                a = 1;
                b = 1;
                d = 2;
            }
            Recipe::PETRI => {
                a = 1;
                c = 2;
                d = 1;
            }
            Recipe::SILENCE => {
                b = 2;
                c = 1;
                e = 1;
            }
        }

        Magimins { a, b, c, d, e }
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, value_name="ingredients.txt", default_value_t=("ingredients.rs".to_owned()))]
    ingredients: String,

    #[arg(short, long, value_enum, value_name="mode", default_value_t=SolveAlgorithm::EXACT)]
    mode: SolveAlgorithm,

    #[arg(short, long, value_enum, value_name="recipe", default_value_t=Recipe::HEALTH)]
    recipe: Recipe,
}

pub struct SolveState {
    ingredients: Arc<Vec<(Ingredient, Option<usize>)>>,
    target: IngredientRatio,
    acc: Arc<Mutex<Vec<PotionRecipe>>>,
    specific_state: SpecificSharedState,
}

#[derive(Clone)]
pub enum SpecificSharedState {
    Exact,
    Approximate {
        global_best_rms: Arc<atomic_float::AtomicF64>,
        thread_best_recipes: Arc<Mutex<Vec<PotionRecipe>>>,
    },
}

fn main() {
    let args = Args::parse();

    let mut target_base = IngredientRatio {
        magimins: args.recipe.to_magimins(),

        taste: 0,
        feel: 0,
        sight: 0,
        smell: 0,
        sound: 0,

        count: 10,
        min: 290,
        max: 575,
        price: 0,
    };

    let mut ingredients = Ingredient::load(&args.ingredients);
    let old_len = ingredients.len();
    ingredients.retain(|(i, _)| match args.mode {
        SolveAlgorithm::EXACT => target_base.is_possible_ingredient(i),
        SolveAlgorithm::APPROXIMATE => true,
    });
    ingredients.sort();

    println!(
        "Found {} ingredients, only {} are candidates",
        old_len,
        ingredients.len()
    );

    let shared_ingredients = Arc::new(ingredients);
    let shared_acc = Arc::new(Mutex::new(Vec::new()));
    let shared_specific_state = match args.mode {
        SolveAlgorithm::APPROXIMATE => {
            // Scale the target ratio up based on the magmins we expect.
            target_base.magimins = target_base.max / &target_base.magimins;
            SpecificSharedState::Approximate {
                global_best_rms: Arc::new(atomic_float::AtomicF64::new(f64::NAN)),
                thread_best_recipes: Arc::new(Mutex::new(Vec::new())),
            }
        }
        _ => SpecificSharedState::Exact,
    };

    rayon::scope(|s| {
        for i in 0..shared_ingredients.len() {
            let state = Arc::new(SolveState {
                ingredients: shared_ingredients.clone(),
                acc: shared_acc.clone(),
                target: target_base.clone(),
                specific_state: shared_specific_state.clone(),
            });
            s.spawn(move |_| {
                let mut ingredients_vec = Vec::new();
                ingredients_vec.reserve_exact(target_base.max);
                println!("starting {} from the top", i);
                enumerate(
                    &(state.ingredients[i..]),
                    state.target.count,
                    &mut ingredients_vec,
                    PotionAttributes::default(),
                    &mut |candidate_ingredients: &[&Ingredient],
                          candidate_ratio: &PotionAttributes|
                     -> bool {
                        // Return false to tell the enumerator to abort this recipe.
                        // First do some common checks that are algorithm agnostic.
                        let target = &state.target;
                        assert!(candidate_ingredients.len() > 0);
                        let candidate_total = candidate_ratio.magimins.total();
                        if candidate_total > target.max {
                            return false;
                        }

                        let last_ingredient_magimins =
                            candidate_ingredients.last().unwrap().mutamin;
                        let remaining_ingredients_count =
                            target.count - candidate_ingredients.len();
                        if (candidate_total
                            + (last_ingredient_magimins * remaining_ingredients_count))
                            < target.min
                        {
                            return false;
                        }

                        if candidate_total < target.min {
                            return true;
                        }

                        // Algorithm specific checks.
                        match &state.specific_state {
                            SpecificSharedState::Exact => {
                                match candidate_ratio.satisfying_ratio(&target) {
                                    None => {
                                        return true;
                                    }
                                    Some(0) => {
                                        return true;
                                    }
                                    Some(_) => {}
                                };
                                let potion_price = candidate_ingredients
                                    .iter()
                                    .fold(0, |p, ingredient| p + ingredient.price);
                                print(
                                    "++ ",
                                    candidate_ingredients.len(),
                                    candidate_total,
                                    candidate_ratio.sense_score(),
                                    potion_price,
                                    candidate_ingredients,
                                );
                                state.acc.lock().unwrap().push(PotionRecipe {
                                    ingredients: candidate_ingredients
                                        .iter()
                                        .map(|&x| x.clone())
                                        .collect(),
                                    attributes: candidate_ratio.clone(),
                                    cost: potion_price,
                                });
                            }
                            &SpecificSharedState::Approximate {
                                ref global_best_rms,
                                ref thread_best_recipes,
                            } => {
                                let mut thread_best_recipes = thread_best_recipes.lock().unwrap();
                                let scaled_expected_ratio_array = state.target.magimins.as_array();

                                let candidate_ratio_magimins_array =
                                    candidate_ratio.magimins.as_array();
                                let new_rms = rms(
                                    scaled_expected_ratio_array,
                                    target.max,
                                    candidate_ratio_magimins_array,
                                );
                                let mut current_rms =
                                    global_best_rms.load(atomic::Ordering::Acquire);
                                // TODO flag to control if bigger is always better.
                                if thread_best_recipes.is_empty() {
                                    let potion_price = candidate_ingredients
                                        .iter()
                                        .fold(0, |p, ingredient| p + ingredient.price);
                                    print(
                                        "++ ",
                                        candidate_ingredients.len(),
                                        candidate_ratio.magimins.total(),
                                        candidate_ratio.sense_score(),
                                        potion_price,
                                        candidate_ingredients,
                                    );
                                    thread_best_recipes.push(PotionRecipe {
                                        ingredients: candidate_ingredients
                                            .iter()
                                            .map(|&x| x.clone())
                                            .collect(),
                                        attributes: candidate_ratio.clone(),
                                        cost: potion_price,
                                    });
                                    return true;
                                } else
                                // If the candidate recipe is at least as good as a recipe we've already found
                                if thread_best_recipes.last().unwrap().ingredients.len()
                                    <= candidate_ingredients.len()
                                {
                                    // But if it's got a worse error than the current global best.
                                    // nb. if current_rms is NaN, as it is when initializing, then this returns false.
                                    if new_rms > current_rms {
                                        // Then project if we can possibly beat the global best with what we've got.
                                        let mut potential_ratio_magimins_array =
                                            candidate_ratio_magimins_array;
                                        let mut useful_deltas: [usize; 5] = [0; 5];
                                        for i in 0..scaled_expected_ratio_array.len() {
                                            if scaled_expected_ratio_array[i] != 0 {
                                                if scaled_expected_ratio_array[i]
                                                    > candidate_ratio_magimins_array[i]
                                                {
                                                    useful_deltas[i] =
                                                        scaled_expected_ratio_array[i].abs_diff(
                                                            candidate_ratio_magimins_array[i],
                                                        );
                                                }
                                            }
                                        }

                                        let mut magimins_remaining = target.max - candidate_total;

                                        loop {
                                            //println!("magimins remaining: {}", magimins_remaining);
                                            assert!(magimins_remaining < target.max);
                                            if magimins_remaining == 0 {
                                                break;
                                            }

                                            let mut delta_indices = [false; 5];
                                            let mut delta_indices_count = 0;
                                            let mut largest_delta = 0;
                                            let mut next_delta = 0;
                                            for i in 0..useful_deltas.len() {
                                                let delta = useful_deltas[i];
                                                if delta > largest_delta {
                                                    next_delta = largest_delta;
                                                    largest_delta = delta;
                                                    delta_indices = [false; 5];
                                                    delta_indices[i] = true;
                                                    delta_indices_count = 1;
                                                } else if delta == largest_delta {
                                                    delta_indices[i] = true;
                                                    delta_indices_count += 1;
                                                } else if delta > next_delta {
                                                    next_delta = useful_deltas[i];
                                                }
                                            }
                                            //println!("delta indices: {}", delta_indices_count);
                                            if delta_indices_count == 0 {
                                                break;
                                            }
                                            //println!("largest delta: {}, next delta: {}", largest_delta, next_delta);
                                            let computed_delta = largest_delta - next_delta;
                                            let needed_magimins =
                                                computed_delta * delta_indices_count;
                                            //println!("needed magimins: {}, magimins remaining: {}", needed_magimins, magimins_remaining);
                                            if needed_magimins <= magimins_remaining {
                                                // Blindly add the delta to every index needing it.
                                                for i in 0..useful_deltas.len() {
                                                    if delta_indices[i] {
                                                        potential_ratio_magimins_array[i] +=
                                                            computed_delta;
                                                        useful_deltas[i] -= computed_delta;
                                                    }
                                                }
                                                magimins_remaining -= needed_magimins;
                                            } else {
                                                // Need to distribute the remainder equally.
                                                let available_delta =
                                                    magimins_remaining / delta_indices_count;
                                                magimins_remaining -=
                                                    available_delta * delta_indices_count;
                                                for i in 0..useful_deltas.len() {
                                                    if delta_indices[i] {
                                                        potential_ratio_magimins_array[i] +=
                                                            available_delta;

                                                        useful_deltas[i] -= available_delta;
                                                        if magimins_remaining > 0 {
                                                            potential_ratio_magimins_array[i] += 1;
                                                            magimins_remaining -= 1;
                                                            useful_deltas[i] -= 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        let potential_rms = rms(
                                            scaled_expected_ratio_array,
                                            target.max,
                                            potential_ratio_magimins_array,
                                        );
                                        println!("potential rms: {}, current best rms: {}", potential_rms, current_rms);
                                        return potential_rms < current_rms;
                                    }
                                    println!(
                                        "total: {}, {}, error: {}",
                                        candidate_ratio.magimins.total(),
                                        candidate_ratio.magimins,
                                        new_rms,
                                    );
                                    let potion_price = candidate_ingredients
                                        .iter()
                                        .fold(0, |p, ingredient| p + ingredient.price);
                                    print(
                                        "++ ",
                                        candidate_ingredients.len(),
                                        candidate_ratio.magimins.total(),
                                        candidate_ratio.sense_score(),
                                        potion_price,
                                        candidate_ingredients,
                                    );
                                    state.acc.lock().unwrap().push(PotionRecipe {
                                        ingredients: candidate_ingredients
                                            .iter()
                                            .map(|&x| x.clone())
                                            .collect(),
                                        attributes: candidate_ratio.clone(),
                                        cost: potion_price,
                                    });
                                    loop {
                                        let old_best_rms = global_best_rms.compare_and_swap(
                                            current_rms,
                                            new_rms,
                                            std::sync::atomic::Ordering::AcqRel,
                                        );
                                        // Either we succeeded.
                                        if current_rms == old_best_rms {
                                            break;
                                        }
                                        current_rms = old_best_rms;
                                        // or someone else already beat us with a better score
                                        if current_rms <= new_rms {
                                            break;
                                        }
                                        // Else we try again.
                                    }
                                }
                            }
                        }
                        return true;
                    },
                );
                ingredients_vec.clear();
                println!("All done with {}!", i);
            });
        }
    });
    /*
    solve(
        &ingredients.as_slice(),
        None,
        &mut candidate_recipe,
        &IngredientRatio::default(),
        &target,
        &mut |candidate_recipe, candidate_ratio, target_ratio| match args.mode {
            SolveAlgorithm::EXACT => {
                match candidate_ratio.satisfying_ratio(&target_ratio) {
                    None => {
                        return;
                    }
                    Some(0) => {
                        return;
                    }
                    Some(s) => {}
                };
                print(
                    "++ ",
                    candidate_ratio.count,
                    candidate_ratio.max,
                    candidate_ratio.sense_score(),
                    candidate_ratio.price,
                    &candidate_recipe,
                );
                acc.push((candidate_recipe, candidate_ratio));
            }
            SolveAlgorithm::APPROXIMATE => {
                if acc.len() == 0 {
                    acc.push((candidate_recipe, candidate_ratio));
                    return;
                }
                let current_best = acc.last().unwrap();
                let current_best_recipe = &current_best.0;
                let current = &current_best.1;

                let current_rms = rms(
                    scaled_expected_ratio.as_array(),
                    target.max,
                    current.magimins.as_array(),
                );
                let new_rms = rms(
                    scaled_expected_ratio.as_array(),
                    target.max,
                    candidate_ratio.magimins.as_array(),
                );
                if current_best_recipe.len() < candidate_recipe.len()
                    || (current_best_recipe.len() == candidate_recipe.len()
                        && new_rms < current_rms)
                {
                    println!(
                        "total: {}, {}, error: {}",
                        candidate_ratio.magimins.total(),
                        candidate_ratio.magimins,
                        new_rms,
                    );
                    print(
                        "",
                        candidate_ratio.count,
                        candidate_ratio.max,
                        candidate_ratio.sense_score(),
                        candidate_ratio.price,
                        &candidate_recipe,
                    );
                    acc.push((candidate_recipe, candidate_ratio));
                }
            }
        },
    );
    acc.sort_by_key(|(_, ratio)| ratio.clone());
    for (names, ratio) in acc.into_iter() {
        print(
            "",
            ratio.count,
            ratio.max,
            ratio.sense_score(),
            ratio.price,
            &names,
        );
    }
     */
}
