pub fn day1(input: String) -> () {
    let elf_detailed_calories: Vec<Vec<i32>> = input
        .split("\n\n")
        .map(
            |chunk| chunk
                .split("\n")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        ).collect();
    let elf_calories: Vec<i32> =
        elf_detailed_calories
        .iter()
        .map(|detailed_calories| detailed_calories.iter().sum())
        .collect();

    let max_calories: i32 =
        elf_calories
        .iter()
        .copied()
        .max().unwrap();
    println!("Max calories: {:?}", max_calories);
    
    let top_three_calories: Vec<i32> = {
        let mut elf_calories_sorted = elf_calories;
        elf_calories_sorted.sort();
        elf_calories_sorted.iter().rev().take(3).copied().collect()
    };

    let total_calories: i32 = top_three_calories.iter().sum();

    println!("Total calories: {:?}", total_calories);
}
