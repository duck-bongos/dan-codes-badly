// Protein
// Calories
// Cost
// Servings = 1

use core::panic;
use ordered_float::OrderedFloat;
use std::fmt::Display;

#[derive(Clone)]
pub struct UxItem<T: Into<f64>, U: Into<f64>, V: Into<f64>, W: Into<f64>> {
    pub protein: T,
    pub calories: U,
    pub cost: V,
    pub servings: W,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroceryItem {
    pub protein: OrderedFloat<f64>,
    pub calories: OrderedFloat<f64>,
    pub cost: OrderedFloat<f64>,
    pub servings: OrderedFloat<f64>,
    pub name: String,
    pub leanness: OrderedFloat<f64>, // lower is better
    pub ppd: OrderedFloat<f64>,      // protein per dollar
    pub lpd: OrderedFloat<f64>,      // leanness per dollar
}

impl<T, U, V, W> UxItem<T, U, V, W>
where
    f64: From<T>,
    f64: From<U>,
    f64: From<V>,
    f64: From<W>,
{
    pub fn new(protein: T, calories: U, cost: V, servings: W, name: String) -> GroceryItem {
        // need to figure out how to panic and raise errors back to UI.
        let _protein: OrderedFloat<f64> = OrderedFloat(protein.into());
        let _calories: OrderedFloat<f64> = OrderedFloat(calories.into());
        let _cost: OrderedFloat<f64> = OrderedFloat(cost.into());
        let _servings: OrderedFloat<f64> = OrderedFloat(servings.into());
        let _leanness: OrderedFloat<f64> = calc_leanness(&_protein, &_calories);
        let _ppd: OrderedFloat<f64> = calc_protein_per_dollar(&_protein, &_cost, &_servings);
        let _lpd: OrderedFloat<f64> =
            calc_leanness_per_dollar(&_protein, &_calories, &_cost, &_servings);

        GroceryItem {
            protein: _protein,
            calories: _calories,
            cost: _cost,
            servings: _servings,
            name: name,
            leanness: _leanness,
            ppd: _ppd,
            lpd: _lpd,
        }
    }

    pub fn to_grocery(self) -> GroceryItem {
        let _protein: OrderedFloat<f64> = OrderedFloat(self.protein.into());
        let _calories: OrderedFloat<f64> = OrderedFloat(self.calories.into());
        let _cost: OrderedFloat<f64> = OrderedFloat(self.cost.into());
        let _servings: OrderedFloat<f64> = OrderedFloat(self.servings.into());
        let _leanness: OrderedFloat<f64> = calc_leanness(&_protein, &_calories);
        let _ppd: OrderedFloat<f64> = calc_protein_per_dollar(&_protein, &_cost, &_servings);
        let _lpd: OrderedFloat<f64> =
            calc_leanness_per_dollar(&_protein, &_calories, &_cost, &_servings);

        GroceryItem {
            protein: _protein,
            calories: _calories,
            cost: _cost,
            servings: _servings,
            name: self.name,
            leanness: _leanness,
            ppd: _ppd,
            lpd: _lpd,
        }
    }
}

impl Display for GroceryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | P: {:.2} Cal: {:.2} Servings: {:.2} - ${:.2}",
            self.name, self.protein, self.calories, self.servings, self.cost,
        )
    }
}

pub fn calc_leanness(
    protein: &OrderedFloat<f64>,
    calories: &OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    /* Lower is better. Sort ascending! */
    if *protein == 0.0 {
        // return Err("Attempted division by 0.0".to_string());
        panic!("Protein cannot be 0.0!");
    }

    let result = calories / protein;
    if result.is_infinite() {
        panic!("Result is infinite");
    } else if result.is_nan() {
        panic!("Result is not a number (NaN).");
    } else {
        result
    }
}

pub fn calc_protein_per_dollar(
    protein: &OrderedFloat<f64>,
    cost: &OrderedFloat<f64>,
    servings: &OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    /* Higher is better. Sort descending! */
    if *cost <= OrderedFloat(0.0) {
        panic!("Cost is {}, must be greater than 0!", cost)
    }

    let result = (protein * servings) / cost;

    if result.is_infinite() {
        // return Err("Result is infinite".to_string());
        panic!("Result is infinite");
    } else if result.is_nan() {
        panic!("Result is not a number (NaN).");
    } else {
        result
    }
}

pub fn calc_leanness_per_dollar(
    protein: &OrderedFloat<f64>,
    calories: &OrderedFloat<f64>,
    cost: &OrderedFloat<f64>,
    servings: &OrderedFloat<f64>,
) -> OrderedFloat<f64> {
    /* Higher is better. Sort ascending! */
    if *cost == 0.0 {
        panic!("Cost: $0.0. Will not attempt division by zero.")
    }

    if *calories == 0.0 {
        // return Err("Attempted division by 0.0".to_string());
        panic!("Calories cannot be 0.0!");
    }

    let _leanness = calc_leanness(&protein, &calories);

    let result = (_leanness * servings) / cost;

    if result.is_infinite() {
        // return Err("Result is infinite".to_string());
        panic!("Result is infinite");
    } else if result.is_nan() {
        panic!("Result is not a number (NaN).");
    } else {
        result
    }
}

pub fn show_leanness(arr: &mut Vec<GroceryItem>) -> &mut Vec<GroceryItem> {
    arr.sort_by(|a, b| a.leanness.cmp(&b.leanness));
    tracing::debug!("\nLeanness scores - Lower is Better");
    // for _item in &arr {
    //     tracing::debug!("{} | {:.2} ", _item, &_item.leanness);
    // }
    arr
}

pub fn show_leanness_per_dollar(arr: &mut Vec<GroceryItem>) {
    arr.sort_by(|a, b| a.lpd.cmp(&b.lpd));
    println!("\nLeanness per dollar");
    for _item in arr {
        println!("{} | {:.2} ", _item, &_item.lpd);
    }
}

pub fn show_protein_per_dollar(arr: &mut Vec<GroceryItem>) {
    arr.sort_by(|a, b| b.ppd.cmp(&a.ppd));
    println!("\nProtein (g) per dollar");
    for _item in arr {
        println!("{} | {:.2} ", _item, &_item.ppd);
    }
}
