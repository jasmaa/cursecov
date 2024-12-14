use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use glob::glob;
use oxc_allocator::Allocator;
use oxc_ast::ast::SourceType;
use oxc_parser::{ParseOptions, Parser};
use regex::Regex;

#[derive(Debug)]
pub struct CountAnalysis {
    comment_count: i32,
    curse_comment_count: i32,
}

fn get_curse_regex() -> Regex {
    let keywords = vec![
        "motherfucking",
        "motherfucker",
        "fucking",
        "fucked",
        "fucker",
        "fuck",
        "crappy",
        "crap",
        "dumbass",
        "ass",
        "shit",
        "bullshit",
    ];
    return Regex::new(format!(r"\b({})\b", keywords.join("|")).as_str()).unwrap();
}

fn count_curse_comments_in_file(path: &Path) -> Result<CountAnalysis, String> {
    let curse_re = get_curse_regex();

    let mut comment_count = 0;
    let mut curse_comment_count = 0;

    let source_text = fs::read_to_string(path).map_err(|err| format!("{}", err))?;
    let source_type = SourceType::from_path(path).map_err(|err| format!("{}", err))?;

    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();

    for comment in ret.program.comments {
        let s = comment.content_span().source_text(&source_text);
        if curse_re.is_match(s) {
            curse_comment_count += 1;
        }
        comment_count += 1;
    }

    Ok(CountAnalysis {
        curse_comment_count: curse_comment_count,
        comment_count: comment_count,
    })
}

fn glob_patterns_to_paths(glob_patterns: &Vec<&str>) -> Result<HashSet<PathBuf>, String> {
    let mut paths: HashSet<PathBuf> = HashSet::new();
    for glob_pattern in glob_patterns {
        for entry in glob(glob_pattern).map_err(|e| format!("{}", e))? {
            let path = entry.map_err(|e| format!("{}", e))?;
            paths.insert(path);
        }
    }
    Ok(paths)
}

fn count_curse_comments(
    include_globs: &Vec<&str>,
    ignore_globs: &Vec<&str>,
) -> Result<CountAnalysis, String> {
    let mut comment_count = 0;
    let mut curse_comment_count = 0;

    let included_paths = glob_patterns_to_paths(include_globs)?;
    let ignored_paths = glob_patterns_to_paths(ignore_globs)?;
    let valid_paths = included_paths.difference(&ignored_paths);

    for path in valid_paths {
        let count_analysis = count_curse_comments_in_file(&path)?;
        comment_count += count_analysis.comment_count;
        curse_comment_count += count_analysis.curse_comment_count;
    }

    Ok(CountAnalysis {
        comment_count,
        curse_comment_count,
    })
}

pub fn run_cursecov(
    include_pattern: String,
    ignore_pattern: String,
    min_coverage: f64,
) -> Result<(), String> {
    let include_globs = include_pattern.split(",").collect::<Vec<&str>>();
    let ignore_globs = ignore_pattern.split(",").collect::<Vec<&str>>();

    let count_analysis = count_curse_comments(&include_globs, &ignore_globs)?;

    let curse_coverage = (100.0 * (count_analysis.curse_comment_count as f64)
        / (count_analysis.comment_count as f64 + 0.0001))
        .floor();

    if curse_coverage >= min_coverage {
        Ok(())
    } else {
        Err(format!(
            "Insufficient curse word coverage: expected {}% but was {}%.",
            min_coverage, curse_coverage
        ))
    }
}
