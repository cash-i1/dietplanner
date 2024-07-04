use std::iter::Sum;

use leptos::{logging::log, *};

#[derive(Debug)]
struct FoodItem {
    name: String,
    calories: f32,
    protein: f32,
}
impl FoodItem {
    pub fn new(name: impl ToString, calories: f32, protein: f32) -> Self {
        FoodItem {
            name: name.to_string(),
            calories,
            protein,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn calories_fmt(&self) -> String {
        format!("{}g", self.calories.clone())
    }
    pub fn protein_fmt(&self) -> String {
        format!("{}g", self.protein.clone())
    }
}

#[component]
fn FoodList() -> impl IntoView {
    let food_items = vec![
        FoodItem::new("egg", 12., 20.),
        FoodItem::new("banana", 129., 10.),
    ];

    view! {
        <div class="food-list">
            <table>
                <tr>
                    <th>food</th>
                    <th>calories</th>
                    <th>protein</th>
                </tr>
                {
                    food_items
                        .iter()
                        .map(|i| view! {
                            <tr>
                                <td>{i.name()}</td>
                                <td>{i.calories_fmt()}</td>
                                <td>{i.protein_fmt()}</td>
                            </tr>
                        })
                        .collect_view()
                }
                {
                    let mut food_items_sum = FoodItem::new("TOTAL", 0., 0.);
                    let _ = food_items
                        .iter()
                        .inspect(|i| {
                            log!("item: {:?}", i);
                            food_items_sum.protein += i.protein;
                            food_items_sum.calories += i.calories;
                        })
                        .for_each(drop); //wtf does this do

                    view! {
                        <tr>
                            <td>TOTAL</td>
                            <td>{food_items_sum.calories_fmt()}</td>
                            <td>{food_items_sum.protein_fmt()}</td>
                        </tr>
                    }
                }
            </table>
        </div>

    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Home/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="root">
            <div class="center-box">
                <h1>diet tracker</h1>
                <FoodList/>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(move || view! { <App/> })
}
