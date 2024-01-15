slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_divide_income(move |string| {
        let ui = ui_handle.unwrap();

        let mut parsed_num: f64 = 0.0;

        match string.trim().parse::<f64>() {
            Ok(value) => parsed_num = value,
            Err(_) => {
                println!("Unable to parse. Resetting variable.");
                
            }
        }

        let tax: f64 = parsed_num * TAXPER;
        let owner: f64 = parsed_num * OWNERPER;
        let profit: f64 = parsed_num * PROFITPER;
        let opex: f64 = parsed_num * OPEXPER;

        let result = format!(
            "Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}",
            { tax },
            { owner },
            { profit },
            { opex }
        );

        ui.set_results(result.into());
    });
    ui.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_100() {
        let result = TAXPER + OWNERPER + PROFITPER + OPEXPER;
        let formatted = f64::trunc(result * 100.0) / 100.0;
        assert_eq!(formatted, 1.00);
    }
}
