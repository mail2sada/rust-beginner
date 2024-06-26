/// Demonstraits constant declaration
const M_TO_CM_RATIO: f64 = 100.0;
const KG_TO_G_RATIO: f64 = 1000.0;

fn convert_meters_to_centimeters(meters: f64) -> f64 {
    meters * M_TO_CM_RATIO
}

fn convert_kilograms_to_grams(kilograms: f64) -> f64 {
    kilograms * KG_TO_G_RATIO
}

fn main() {
    println!("Demo:Constant declaraion and usage");
    let length_m = 2.5;
    let weight_kg = 1.8;

    println!("Length in centimeters: {}", convert_meters_to_centimeters(length_m));
    println!("Weight in grams: {}", convert_kilograms_to_grams(weight_kg));
}
