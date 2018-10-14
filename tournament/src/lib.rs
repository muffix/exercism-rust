use std::collections::HashMap;

enum Result {
    Win,
    Loss,
    Tie,
}

pub fn tally(input: &str) -> String {
    let mut results: HashMap<String, (usize, usize, usize)> = HashMap::new();

    for line in input.split("\n") {
        if line == "" {
            continue;
        }

        let fields = line.split(';').collect::<Vec<&str>>();
        match fields[2] {
            "win" => {
                record_result(Result::Win, fields[0], &mut results);
                record_result(Result::Loss, fields[1], &mut results);
            }
            "loss" => {
                record_result(Result::Win, fields[1], &mut results);
                record_result(Result::Loss, fields[0], &mut results);
            }
            "draw" => {
                record_result(Result::Tie, fields[0], &mut results);
                record_result(Result::Tie, fields[1], &mut results);
            }
            _ => panic!("Invalid result"),
        };
    }

    generate_table(results)
}

fn record_result(result: Result, team: &str, scores: &mut HashMap<String, (usize, usize, usize)>) {
    let team_statistics = scores.entry(team.to_string()).or_insert((0, 0, 0));
    match result {
        Result::Win => team_statistics.0 += 1,
        Result::Loss => team_statistics.1 += 1,
        Result::Tie => team_statistics.2 += 1,
    }
}

fn generate_table(results: HashMap<String, (usize, usize, usize)>) -> String {
    let mut table = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    let mut teams = results.iter().collect::<Vec<_>>();

    teams.sort_by_key(|&(name, &(w, _, t))| (std::usize::MAX - w * 3 - t, name));

    for &(name, &(win, loss, tie)) in teams.iter() {
        table.push(format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            name,
            win + loss + tie,
            win,
            tie,
            loss,
            3 * win + tie
        ));
    }

    table.join("\n")
}
