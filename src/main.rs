use std::iter::Sum;

use leptos::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
    let (food_items, set_food_items, _) =
        use_local_storage::<Vec<FoodItem>, JsonCodec>("food_items");

    view! {
        <div class="food-list">
            <table>
                <tr class="food-header">
                    <th>food</th>
                    <th>calories</th>
                    <th>protein</th>
                </tr>
                {
                    move || {
                        food_items.get()
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
                }
                {
                    move || {
                        let mut food_items_sum = FoodItem::new("TOTAL", 0., 0.);
                        let _ = food_items.get()
                            .iter()
                            .inspect(|i| {
                                food_items_sum.protein += i.protein;
                                food_items_sum.calories += i.calories;
                            })
                            .for_each(drop); //wtf does this do

                        view! {
                            <tr class="total">
                                <td>TOTAL</td>
                                <td>{food_items_sum.calories_fmt()}</td>
                                <td>{food_items_sum.protein_fmt()}</td>
                            </tr>
                        }
                    }
                }
            </table>
        </div>

    }
}

#[component]
fn AddFoodItem() -> impl IntoView {
    let (food_items, set_food_items, reset_food_items) =
        use_local_storage::<Vec<FoodItem>, JsonCodec>("food_items");
    let (food_item, set_food_item) = create_signal(FoodItem::new("", 0., 0.));

    view! {
        <h2>add an item</h2>
        <div class="add-food-item">
            <button
                on:click=move |_| {
                    reset_food_items()
                }
            >reset diet tracker</button>
            <table>
                <tr>
                    <td>
                        <input
                            type="text"
                            placeholder="food name"
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_food_item.update(|i| i.name = val);
                            }
                        />
                    </td>
                    <td>
                        <input
                            type="number"
                            placeholder="calories"
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_food_item.update(|i| i.calories = val.parse::<f32>().unwrap_or(0.));
                            }
                        />
                    </td>
                    <td>
                        <input
                            type="number"
                            placeholder="protein"
                            on:input=move |ev| {
                                let val = event_target_value(&ev);
                                set_food_item.update(|i| i.protein = val.parse::<f32>().unwrap_or(0.));
                            }
                        />
                    </td>
                </tr>
            </table>
            <button
                on:click=move |_| {
                    if food_item.get().name.is_empty()
                    || food_item.get().calories == 0.0 
                    || food_item.get().protein == 0.0 {

                    } else {
                        set_food_items.update(|i| i.push(food_item.get()));
                    }
                }
            >add</button>
            <br/>
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
                <div id="notification-box"></div>
                <h1>diet tracker</h1>
                <AddFoodItem/>
                <FoodList/>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(move || view! { <App/> })
}
