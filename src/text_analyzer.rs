use clap::Args;
use std::collections::HashMap;
use std::fs;

#[derive(Args)]
pub struct AnalyzerOptions {
    #[arg(short, long, help = "Path to the input text file")]
    input: String,

    #[arg(short, long, default_value_t = 5, help = "Number of top frequent words to display")]
    top: usize,
}

pub fn run_text_analyzer(options: AnalyzerOptions) {
    match analyze_text(&options.input, options.top) {
        Ok(_) => println!("Analysis complete!"),
        Err(e) => eprintln!("Error analyzing text: {}", e),
    }
}

fn analyze_text(file_path: &str, top_n: usize) -> Result<(), std::io::Error> {
    // Read the file content
    let content = fs::read_to_string(file_path)?;

    // Perform analysis
    let word_count = count_words(&content);
    let char_count = count_characters(&content);
    let line_count = content.lines().count();
    let most_frequent_words = find_most_frequent_words(&content, top_n);

    // Display results
    println!("Analysis Results:");
    println!("Total Words: {}", word_count);
    println!("Total Characters (with spaces): {}", char_count.with_spaces);
    println!("Total Characters (without spaces): {}", char_count.without_spaces);
    println!("Total Lines: {}", line_count);
    println!("Top {} Frequent Words:", top_n);
    for (word, count) in most_frequent_words {
        println!("  {}: {}", word, count);
    }

    Ok(())
}

struct CharCount {
    with_spaces: usize,
    without_spaces: usize,
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

fn count_characters(content: &str) -> CharCount {
    let with_spaces = content.chars().count();
    let without_spaces = content.chars().filter(|c| !c.is_whitespace()).count();
    CharCount {
        with_spaces,
        without_spaces,
    }
}

fn find_most_frequent_words(content: &str, top_n: usize) -> Vec<(String, usize)> {
    let mut word_freq: HashMap<String, usize> = HashMap::new();

    for word in content.split_whitespace() {
        let clean_word = word.to_lowercase();
		let clean_word = clean_word.trim_matches(|c: char| !c.is_alphanumeric());
		*word_freq.entry(clean_word.to_string()).or_insert(0) += 1;

    }

    let mut freq_vec: Vec<(String, usize)> = word_freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    freq_vec.into_iter().take(top_n).collect()
}