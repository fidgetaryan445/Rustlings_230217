fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) ->i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}
