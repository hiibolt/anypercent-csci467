use std::collections::BTreeMap;
#[derive(Debug, Clone)]
struct QuestionAnswerTree {
    answer: Option<String>,
    children: BTreeMap<String, QuestionAnswerTree>,
}
fn place_words (
    qa_tree: &mut BTreeMap<String, QuestionAnswerTree>,
    words: &[&str],
    answer: String,
) {
    let answer_opt = (words.len() == 1).then_some(answer.clone());
    let current = qa_tree.entry(words[0].to_string())
        .or_insert(QuestionAnswerTree {
            answer: answer_opt,
            children: BTreeMap::new(),
        });
    if words.len() == 1 {
        return;
    }
    
    place_words (
        &mut current.children,
        &words[1..],
        answer
    );
}
fn simplify_tree (
    qa_tree: &mut BTreeMap<String, QuestionAnswerTree>,
) {
    for key in qa_tree.clone().keys() {
        let mut key = key.clone();
        while qa_tree.get(&key).expect(":<").children.len() == 1 {
            let qa_subtree = qa_tree.get_mut(&key).expect(":<");
            let (child_key, child_qa) = qa_subtree.children.iter_mut().next().unwrap();

            let new_children= std::mem::take(&mut child_qa.children);
            let new_key = key.to_string() + " " + child_key;
            let new_answer = child_qa.answer.clone();
            
            
            // Remove the old entry
            qa_tree.remove(&key);

            // Insert the new entry with the combined key
            qa_tree.insert(new_key.clone(), QuestionAnswerTree {
                answer: new_answer,
                children: new_children,
            });

            key = new_key;
        }
        
        simplify_tree(&mut qa_tree.get_mut(&key).expect(":<").children);
    }
}
fn reduce_answer_qa_keys_to_first_word (
    qa_tree: &mut BTreeMap<String, QuestionAnswerTree>,
) {
    for key in qa_tree.clone().keys() {
        let mut key = key.clone();

        if qa_tree.get(&key).expect(":<").children.is_empty() {
            let qa_subtree = qa_tree.get_mut(&key).expect(":<");
            if let Some(answer) = qa_subtree.answer.clone() {
                // Remove the old entry
                qa_tree.remove(&key);

                // Insert the new entry with the first word as the key
                let first_word = key.split_whitespace().next().unwrap().to_string();
                qa_tree.insert(first_word.clone(), QuestionAnswerTree {
                    answer: Some(answer),
                    children: BTreeMap::new(),
                });
                key = first_word;
            }
        }
        
        reduce_answer_qa_keys_to_first_word(&mut qa_tree.get_mut(&key).expect(":<").children);
    }
}
fn reserialize_qa_tree(
    qa_tree: &BTreeMap<String, QuestionAnswerTree>,
) -> Vec<String> {
    let mut result = Vec::new();
    for (key, qa) in qa_tree {
        if let Some(answer) = &qa.answer {
            result.push(format!("{} -- {}", key, answer));
        }
        if !qa.children.is_empty() {
            let child_strings = reserialize_qa_tree(&qa.children);
            for child in child_strings {
                result.push(format!("{} {}", key, child));
            }
        }
    }
    result
}
fn generate_output (
    input_file: &str,
    output_file: &str,
) {
    let data_str = std::fs::read_to_string(input_file)
        .expect("Failed to read input file");
    let data = data_str
        .split("~~")
        .map(|s| {
            s.split("--")
                .map(|s| s.trim().split(" ").collect())
                .collect::<Vec<Vec<&str>>>()
        })
        .collect::<Vec<Vec<Vec<&str>>>>();
    
    let mut qa_tree = BTreeMap::new();
    for qa_vec in &data {
        place_words(
            &mut qa_tree,
            &qa_vec[0],
            qa_vec[1].join(" ").to_string(),
        );
    }

    simplify_tree(&mut qa_tree);
    reduce_answer_qa_keys_to_first_word(&mut qa_tree);

    let result = reserialize_qa_tree(&qa_tree);
    std::fs::write(output_file, result.join("\n"))
        .expect("Failed to write output file");
}
fn main() {
    let input_files = vec!(
        ("assets/input-midterm.txt", "assets/output-midterm.txt"),
        ("assets/input-final.txt", "assets/output-final.txt"),
    );
    
    for (input_file, output_file) in input_files {
        generate_output(input_file, output_file);
    }
}
