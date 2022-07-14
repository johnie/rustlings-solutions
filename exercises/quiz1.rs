// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the quantity bought. No hints this time!

// Put your function here!
// fn calculate_apple_price {
fn calculate_apple_price(apples: i32) -> i32 {
    let mut rustbucks: i32 = 2;
    if apples <= 40 {
        rustbucks * apples
    } else {
        let rustbucks = 1;
        rustbucks * apples
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(40);
    let price3 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}
