pub fn sum(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        println!("Line {}", line);
        let mut steps: Vec<Vec<i32>> = Vec::new();
        let mut lasts: Vec<i32> = Vec::new();

        let hist: Vec<i32> = line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        let mut act = hist.clone();
        lasts.push(hist.last().unwrap().clone());
        let mut doit = true;
        let mut differ = false;
        while doit {
            println!("Act {:?}", act);
            let mut diffs: Vec<i32> = Vec::new();
            let mut diff = 0;
            for i in 0..act.len() - 1 {
                diff = act[i + 1] - act[i];
                print!("-> {}-{}={} ", act[i + 1], act[i], diff);
                diffs.push(diff);
                if i > 0 && diffs[i - 1] != diffs[i] {
                    differ = true;
                }
                println!("differ={}", differ)
            }
            lasts.push(diff);
            steps.push(act);
            act = diffs;
            doit = differ;
            differ = false;
        }

        println!("Calc {:?}", lasts);
        let mut val = 0;
        for last in lasts.iter().rev() {
            val += last;
            println!("last {}", last)
        }
        print!("Sum: {} += {}", sum, val);
        sum += val;
        println!(" = {}", sum);
    }
    sum
}
