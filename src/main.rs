slint::include_modules!();

const HOURS_PER_MIN: f64 = 1.0/60.0;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_calc_real_wage(move |wage, commute, efficiency, gas| {
        let wage: f64 = wage.trim().parse().unwrap_or(0.0);
        let commute: f64 = commute.trim().parse().unwrap_or(0.0);
        let efficiency: f64 = efficiency.trim().parse().unwrap_or(1.0);
        let gas: f64 = gas.trim().parse().unwrap_or(0.0);

        // assume 8 hours per workday
        let gross_wage = wage * 8.0;
        let gas_cost = gas * commute / efficiency;
        let net_wage = gross_wage - gas_cost;

        // assume average speed is 30 mph, so double commute length to get time in minutes
        let commute_time = commute * 2.0 * HOURS_PER_MIN;
        let effective_hours = 8.0 + commute_time*2.0;
        let real_wage = net_wage / effective_hours;

        ui_handle.unwrap().set_real_wage(format!("${:.2}/h over {:.2} hours", real_wage, effective_hours).into());
    });

    ui.run()
}
