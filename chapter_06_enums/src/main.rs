#[derive(Debug)]
enum UsState {
    Alabama, Alaska, // ... 他の州 ...
}

// Coin EnumにQuarter(UsState)を追加
enum Coin {
    Penny, Nickel, Dime,
    Quarter(UsState), // QuarterにはUsStateの情報が紐付いている
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // ✨ここがポイント！stateという変数に束縛する
            println!("State quarter from {:?}!", state); // 取り出したstate変数を使える
            25
        },
    }
}

fn main() {
    // 実際にコインを作成してテスト
    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alaska);

    println!("Penny value: {}", value_in_cents(coin1));
    println!("Quarter value: {}", value_in_cents(coin2));
}