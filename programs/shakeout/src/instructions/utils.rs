pub fn calculate_75_percent(amount: u32) -> u32 {
    // Scale up by 100
    let scaled_amount = amount * 100;
    // Multiply by 75 (for 75%)
    let seventy_five_percent = scaled_amount * 75;
    // Scale back down
    seventy_five_percent / 100
}