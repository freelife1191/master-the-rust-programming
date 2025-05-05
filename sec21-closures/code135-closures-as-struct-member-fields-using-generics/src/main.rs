struct TaxCalculator<F: Fn(f32) -> f32> {
    calculation: F,
}

impl<T: Fn(f32) -> f32> TaxCalculator<T> {
    fn new(calculation: T) -> TaxCalculator<T> {
        TaxCalculator { calculation }
    }

    fn calculate(&self, amount: f32) -> f32 {
        (self.calculation)(amount)
    }
}
fn main() {
    let mut vat_calculator = TaxCalculator::new(|amount| amount * 0.2);
    let income_tax_calculator = TaxCalculator::new(|amount| amount * 0.3);
    // let calculators: Vec<TaxCalculator> = vec![vat_calculator, income_tax_calculator]; // Error
    println!("{}", vat_calculator.calculate(1000_f32));
    println!("{}", income_tax_calculator.calculate(1000_f32));

    // vat_calculator = TaxCalculator::new(|amount| amount * 0.5); // Error
    // println!("{}", vat_calculator.calculate(1000f32));
}
