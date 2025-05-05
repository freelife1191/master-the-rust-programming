# Methods to Store a closure in a struct

1. As a trait object
2. **As a generic struct**

## Closures and generic structs

구조체 필드에 클로저를 저장하는 또 다른 일반적인 방법은 특성 경계가 있는 제네릭을 사용하는 것입니다. 
`Box<dyn Trait>`를 사용하는 이전 방법과 달리 이 방법은 정적 디스패치를 사용합니다. 이는 더 빠를 수 있지만 구조체 자체를 일반화합니다.


## Static dispatch

중요한 점은 정적 디스패치 덕분에 Rust 컴파일러가 컴파일 타임에 어떤 특정 함수를 호출할지 알기 때문에 동적 조회가 필요하지 않다는 것입니다.  
함수(또는 메소드)를 데이터 유형과 직접 연결하므로 매우 효율적인 기계 코드 생성이 가능합니다.


```rust
// 각 클로저는 고유한 익명 유형을 갖습니다.
struct ClosureType_vat;
struct ClosureType_income_tax;

// 각 클로저에 대해 Fn 특성을 구현합니다.
impl Fn(f32) -> f32 for ClosureType_vat {
    fn call(&self, amount: f32) -> f32 {
        amount * 0.2
    }
}

impl Fn(f32) -> f32 for ClosureType_income_tax {
    fn call(&self, amount: f32) -> f32 {
        amount * 0.3
    }
}

// `Tax Calculator`는 이제 각 `closure`에 대해 하나씩 두 가지 유형이 됩니다.
struct TaxCalculator_vat {
    calculation: ClosureType_vat,
}

struct TaxCalculator_income_tax {
    calculation: ClosureType_income_tax,
}

// 각 클로저 유형에 대한 TaxCalculator 메소드 구현
impl TaxCalculator_vat {
    fn new(calculation: ClosureType_vat) -> Self {
        TaxCalculator_vat { calculation }
    }
    
    fn calculate(&self, amount: f32) -> f32 {
        self.calculation.call(amount)
    }
}

impl TaxCalculator_income_tax {
    fn new(calculation: ClosureType_income_tax) -> Self {
        TaxCalculator_income_tax { calculation }
    }
    
    fn calculate(&self, amount: f32) -> f32 {
        self.calculation.call(amount)
    }
}

fn main() {
    let vat_calculator = TaxCalculator_vat::new(ClosureType_vat);
    let income_tax_calculator = TaxCalculator_income_tax::new(ClosureType_income_tax);
    
    println!("{}", vat_calculator.calculate(1000f32));
    println!("{}", income_tax_calculator.calculate(1000f32));
}
```