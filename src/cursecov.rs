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
pub struct FileCountAnalysis {
    path: PathBuf,
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

fn count_curse_comments_in_file(path: &Path) -> Result<FileCountAnalysis, String> {
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

    Ok(FileCountAnalysis {
        path: PathBuf::from(path),
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
) -> Result<Vec<FileCountAnalysis>, String> {
    let included_paths = glob_patterns_to_paths(include_globs)?;
    let ignored_paths = glob_patterns_to_paths(ignore_globs)?;
    let valid_paths = included_paths.difference(&ignored_paths);

    let mut count_analyses: Vec<FileCountAnalysis> = Vec::new();
    for path in valid_paths {
        let count_analysis = count_curse_comments_in_file(&path)?;
        count_analyses.push(count_analysis);
    }

    Ok(count_analyses)
}

fn generate_coverage_table(
    count_analyses: &Vec<FileCountAnalysis>,
    total_curse_coverage: f64,
) -> String {
    let mut s = String::new();

    s.push_str(
        format!(
            "| {:<80} | {:<8} | {:<8} | {:<3} |\n",
            "file", "# curse", "# total", "%"
        )
        .as_str(),
    );

    s.push_str(format!("| {:-<80} | {:-<8} | {:-<8} | {:-<3} |\n", "", "", "", "").as_str());

    for count_analysis in count_analyses {
        let curse_coverage = (100.0 * (count_analysis.curse_comment_count as f64)
            / (count_analysis.comment_count as f64 + 0.0001))
            .floor();

        let file_name = count_analysis.path.display().to_string();

        let displayed_file_name = if file_name.len() > 60 {
            let (end1, _) = file_name.char_indices().nth(30).unwrap();
            let (start2, _) = file_name.char_indices().nth(file_name.len() - 30).unwrap();
            format!("{}...{}", &file_name[..end1], &file_name[start2..])
        } else {
            file_name
        };

        s.push_str(
            format!(
                "| {:<80} | {:<8} | {:<8} | {:<3} |\n",
                displayed_file_name,
                count_analysis.curse_comment_count,
                count_analysis.comment_count,
                curse_coverage
            )
            .as_str(),
        );
    }

    s.push('\n');

    s.push_str(format!("Total curse coverage: {}%", total_curse_coverage).as_str());

    return s;
}

fn calculate_total_curse_coverage(count_analyses: &Vec<FileCountAnalysis>) -> f64 {
    let curse_comment_count: i32 = count_analyses.iter().map(|x| x.curse_comment_count).sum();
    let comment_count: i32 = count_analyses.iter().map(|x| x.comment_count).sum();
    (100.0 * (curse_comment_count as f64) / (comment_count as f64 + 0.0001)).floor()
}

pub fn run_cursecov(
    include_pattern: String,
    ignore_pattern: String,
    min_coverage: f64,
    verbose: bool,
) -> Result<(), String> {
    let include_globs = include_pattern.split(",").collect::<Vec<&str>>();
    let ignore_globs = ignore_pattern.split(",").collect::<Vec<&str>>();

    let count_analyses = count_curse_comments(&include_globs, &ignore_globs)?;

    let total_curse_coverage = calculate_total_curse_coverage(&count_analyses);

    if verbose {
        let table = generate_coverage_table(&count_analyses, total_curse_coverage);
        println!("{}", table);
    }

    if total_curse_coverage >= min_coverage {
        Ok(())
    } else {
        Err(format!(
            "Insufficient curse word coverage: expected {}% but was {}%.",
            min_coverage, total_curse_coverage
        ))
    }
}
