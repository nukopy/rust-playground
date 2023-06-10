use std::num::ParseIntError;

fn parse(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e: ParseIntError| e.to_string())
}

fn reciprocal(n: i32) -> Result<f64, String> {
    if n != 0 {
        Ok(1.0 / n as f64)
    } else {
        Err(String::from("Cannot compute the reciprocal of zero"))
    }
}

pub fn test_map() {
    let nums = vec![1, 2, 3, 4, 5];
    let squared_nums: Vec<_> = nums.iter().map(|x| x * x).collect();
    println!("{:?}", squared_nums); // 出力: [1, 4, 9, 16, 25]

    let strings = vec!["1", "2", "3", "4", "5"];
    let nums: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| {
            println!("s: {}", s);
            s.parse::<i32>()
        })
        .collect();
    match nums {
        Ok(nums) => println!("{:?}", nums),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn test_combinator() {
    let s = "2";
    let rec = parse(s).and_then(|n| reciprocal(n));
    let rec = parse(s).and_then(reciprocal);
    // let rec = parse(s)?.and_then(reciprocal);
    // let rec = parse(s)?;
    // let rec = parse(s).and_then(|n| {
    //     println!("n: {}", n);
    //     reciprocal(n)
    // });

    match rec {
        Ok(rec) => println!("{}", rec),
        Err(e) => println!("Error: {}", e),
    }
}
