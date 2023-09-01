// Function to calculate the base price based on the number of current holders.
// The price increases linearly with the number of holders up to a threshold,
// after which the increase becomes constant.
//
// Parameters:
// - current_holders: Number of current shareholders.
//
// Returns:
// - f64: Calculated base price as a floating-point number.
fn base_price_from_holders(current_holders: u32) -> f64 {
    if current_holders <= 10 {
        0.1 * current_holders as f64
    } else {
        (current_holders as f64 - 10.0) + 1.0
    }
}

// Function to determine the share price using a dual-phase pricing algorithm.
// The price is influenced by the volume of shares traded, how long since the last trade,
// and a base price derived from the number of current holders.
//
// Parameters:
// - current_holders: Number of current shareholders.
// - current_volume: Current volume of shares traded.
// - average_volume: Average volume of shares traded.
// - time_since_last_trade: Time (in hours) since the last trade was executed.
//
// Returns:
// - f64: Calculated share price as a floating-point number.
fn dual_phase_pricing(current_holders: u32, current_volume: f64, average_volume: f64, time_since_last_trade: f64) -> f64 {
    // Adjustment factor to account for trading volume. A higher current volume compared to
    // the average volume will lead to a higher share price.
    let volume_adjustment_factor = 0.01;

    // Adjustment factor to account for trading inactivity. If no trades occur for a certain
    // threshold, the share price will decrease.
    let inactivity_adjustment_factor = 0.005;
    let inactivity_threshold = 24.0; 

    // Calculate the base price using the current number of shareholders.
    let base_price = base_price_from_holders(current_holders);

    // Calculate the ratio of the current volume to the average volume.
    let volume_ratio = current_volume / average_volume;

    // Adjust the price based on the volume ratio or trading inactivity.
    if time_since_last_trade > inactivity_threshold {
        base_price * (1.0 - inactivity_adjustment_factor)
    } else {
        base_price * (1.0 + volume_adjustment_factor * volume_ratio)
    }
}

// The main function to simulate and print the share prices during different trading scenarios.
fn main() {
    // An arbitrary value for average volume. Can be set based on historical data or other metrics.
    let average_volume = 7.0;

    // Simulating a "pump" phase where the number of shareholders increases.
    // Printing the share prices during this phase.
    let pump_holders = [1, 10, 100, 10_000];
    for &holders in pump_holders.iter() {
        let price = dual_phase_pricing(holders, 10.0, average_volume, 1.0);
        println!("Price during pump with {} holders: {} SOL", holders, price);
    }

    // Simulating a "dump" phase where the number of shareholders decreases.
    // Printing the share prices during this phase.
    let dump_holders = [50, 80, 30, 20];
    for &holders in dump_holders.iter() {
        let price = dual_phase_pricing(holders, 5.0, average_volume, 12.0);
        println!("Price during dump with {} holders: {} SOL", holders, price);
    }
}
