#![feature(entry_and_modify)]
use std::collections::HashMap;

use std::cmp::Ordering;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default, Eq, PartialEq)]
struct Team {
    name: String,
    wins: u32,
    losses: u32,
    drawns: u32,
    matches: u32,
}

fn oposite(result: &str) -> &str {
    match result {
        "win" => "loss",
        "loss" => "win",
        other => other,
    }
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            wins: 0,
            losses: 0,
            drawns: 0,
            matches: 0,
        }
    }

    pub fn points(&self) -> u32 {
        self.wins * 3 + self.drawns
    }

    pub fn record_result(&mut self, result: &str) {
        self.matches += 1;

        match result {
            "loss" => self.losses += 1,
            "win" => self.wins += 1,
            "draw" => self.drawns += 1,
            _ => {}
        };
    }

    pub fn summary(&self) -> String {
        return format!(
            "{:30} |  {} |  {} |  {} |  {} |  {}",
            self.name,
            self.matches,
            self.wins,
            self.drawns,
            self.losses,
            self.points()
        );
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Team) -> Ordering {
        match other.points().cmp(&self.points()) {
            Ordering::Equal => self.name.partial_cmp(&other.name).unwrap(),
            other => other,
        }
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Team) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
struct ScoreTable<'a> {
    registry: HashMap<&'a str, Team>,
}

impl<'a> ScoreTable<'a> {
    pub fn record_match(&mut self, one_match: &'a str) {
        for row in one_match.split(';').collect::<Vec<&str>>().chunks(3) {
            let team_1_name = row[0];
            let team_2_name = row[1];
            let result = row[2];

            self.team(team_1_name).record_result(result);
            self.team(team_2_name).record_result(oposite(result));
        }
    }

    fn team(&mut self, team_name: &'a str) -> &mut Team {
        self.registry.entry(team_name).or_insert_with(
            || Team::new(team_name),
        )
    }

    fn teams(&self) -> Vec<&Team> {
        let mut result: Vec<&Team> = self.registry.values().collect();
        result.sort();
        result
    }
}

impl<'a> std::convert::From<&'a str> for ScoreTable<'a> {
    fn from(input: &'a str) -> ScoreTable<'a> {
        let mut score_table = ScoreTable::default();

        for one_match in input.split('\n') {
            if one_match.is_empty() {
                continue;
            };
            score_table.record_match(one_match);
        }

        score_table
    }
}

pub fn tally(input: &str) -> String {
    let header = String::from(HEADER);

    let summaries: String = ScoreTable::from(input)
        .teams()
        .iter()
        .map(|team| team.summary())
        .collect::<Vec<_>>()
        .join("\n");

    if summaries.is_empty() {
        return header;
    }

    header + "\n" + &summaries
}
