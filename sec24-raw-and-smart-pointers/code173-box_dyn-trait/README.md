## Dynamically Sized Types : Trait objects

1.  `Trait objects`는 `&dyn Trait` 또는 `&mut dyn Trait`와 같은 `references`를 통해 일반적으로 사용된다
2.  `Trait objects`는 `Box<dyn Trait>`와 같은 `smart pointer`를 통해서도 사용될 수 있다

```rust
// `Trait objects`는 `DSTs`이다
trait InterestCalculator {
    fn calculate_interest(&self) -> f64;
}

struct SavingsAccount {
    balance: f64,
}

impl InterestCalculator for SavingsAccount {
    fn calculate_interest(&self) -> f64 {
        self.balance * 0.03
    }
}

struct FixedDeposit {
    amount: f64,
    duration: f64,
}

impl InterestCalculator for FixedDeposit {
    fn calculate_interest(&self) -> f64 {
        self.amount * 0.05 * self.duration
    }
}

// automatic coercion form `Box<T>` to `Box<dyn Trait>`
// fn display_interest (account: &dyn InterestCalculator) {
fn display_interest (account: Box<dyn InterestCalculator>) {
    println!("Interest: {}", account.calculate_interest());
}

fn main() {
    // let savings = SavingsAccount { balance: 1000.0 };
    let savings: Box<SavingsAccount> = Box::new(SavingsAccount { balance: 1000.0 });
    display_interest(&savings);

    // let fixed = FixedDeposit { amount: 5000.0, duration: 2.0 };
    let fixed: Box<FixedDeposit> = Box::new(FixedDeposit { amount: 5000.0, duration: 2.0 });
    display_interest(&fixed);
}
```

```bash
Interest: 30
Interest: 500
```