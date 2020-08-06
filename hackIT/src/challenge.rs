use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::env;
use std::fs::{read_dir, read_to_string, DirEntry};
use std::path::PathBuf;

/* Container for a challenge and its meta-data

# Examples

```
    let ch = Challenge {
        id: "count",
        lvl: 2,
        name: "count with me!",
        desc: "Help the count count numbers! please count to n",
        spec: "Answere in the format 1,2,..,n",
        scen: vec!(("10","1,2,3,4,5,6,7,8,9,10")),
    };
```
*/
#[derive(Serialize,Deserialize, Debug, PartialEq, Eq)]
pub struct Challenge {
    id: String,
    lvl: u8,
    name: String,
    desc: String,
    spec: String,
    scen: Vec<(String, String)>,
}

/*
    Map of challenges, indexed by thier id, located in memory
*/

pub type Challenges = HashMap<String, Challenge>;

/*
    Parses challenges for a directory, and returns them in a hashmap indexed by their ids.

    dir : &str is the path to the challenge directory from the relative directory of the executor
*/

pub fn load_challenges(dir: &str) -> Challenges {
    let mut challenges: Challenges = HashMap::new();

    let challenges_path = env::current_dir().unwrap().join(dir);
    if let Ok(entries) = read_dir(challenges_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                load_entry(entry, &mut challenges);
            }
        }
    }
    challenges
}

/*
    Attempts to find a info.toml in a directory. If it does it parses it and the directory into a Challenge and inserts it into Challenges
*/

fn load_entry(entry: DirEntry, challenges: &mut Challenges) {
    let info_path = entry.path().join("info.toml");

    if let Ok(info) = read_to_string(&info_path) {
        let mut challenge: Challenge = toml::from_str(&info).unwrap();
        let qa = load_qa_files(&info_path, &challenge.scen);

        challenge.scen = qa;
        challenges.insert(challenge.id.clone(), challenge);
    }
}

/*
    Iterates over the list of files in scen, and returns a vector containing the contents of the files in pairs
*/

fn load_qa_files(root: &PathBuf, qa_files: &[(String, String)]) -> Vec<(String, String)> {
    let mut qa = Vec::<(String, String)>::new();

    for (q_file, a_file) in qa_files {
        let q_path = root.with_file_name(q_file);
        let a_path = root.with_file_name(a_file);

        let q = read_to_string(&q_path);
        let a = read_to_string(&a_path);

        match (q, a) {
            (Ok(q), Ok(a)) => qa.push((q.to_string(), a.to_string())),
            _ => {
                eprintln!(
                    "[Warning] load_qa_files could not load files {},{}",
                    q_path.to_str().unwrap(),
                    a_path.to_str().unwrap()
                );
                continue;
            }
        }
    }
    qa
}

/*
    Compares an answer with the answer given.
*/

fn verify_answer(
    challenges: &Challenges,
    id: &str,
    scn_id: usize,
    answer: &str,
) -> Result<bool, String> {
    if let Some(ch) = challenges.get(id) {
        if let Some(reference) = ch.scen.get(scn_id) {
            Ok(reference.1 == answer)
        } else {
            Err(format!(
                "[Error] in verify-answer: challenge does not have scene id {}",
                scn_id
            ))
        }
    } else {
        Err(format!(
            "[Error] in verify-answer: challenge does not exist in challenges with id {}",
            id
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_challenges() {
        let test_ch = Challenge {
            id: "kuwa".to_string(),
            lvl: 2,
            name: "Whaaaa".to_string(),
            desc: "This ... is requiem".to_string(),
            spec: "The answere is the input repeated on 8 rows".to_string(),
            scen: vec![(
                "whaa\n".to_string(),
                "whaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\n".to_string(),
            )],
        };

        let cs = load_challenges("test_challenges");
        assert_eq!(cs[&test_ch.id], test_ch);
        assert!(verify_answer(
            &cs,
            &test_ch.id,
            0,
            "whaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\n"
        )
        .unwrap());
    }

    #[test]
    fn test_verify_answer() {
        let test_ch = Challenge {
            id: "kuwa".to_string(),
            lvl: 2,
            name: "Whaaaa".to_string(),
            desc: "This ... is requiem".to_string(),
            spec: "The answere is the input repeated on 8 rows".to_string(),
            scen: vec![(
                "whaa\n".to_string(),
                "whaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\n".to_string(),
            )],
        };

        let cs = load_challenges("test_challenges");
        assert!(verify_answer(
            &cs,
            &test_ch.id,
            0,
            "whaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\nwhaa\n"
        )
        .unwrap());
    }
}
