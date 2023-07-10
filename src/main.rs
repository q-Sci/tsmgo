use glob::glob;
use std::fs;

fn main() {
    // create list of all files inside the folder
    let mut sgf_files: Vec<String> = Vec::new();
    for file in glob("./sgf/*.sgf").expect("Failed to read glob pattern") {
        sgf_files.push(file.unwrap().display().to_string());
    }

    let mut output_data: Vec<String> = Vec::new(); // content written to json file, not formatted yet

    // print the content of each sgf file
    for file in sgf_files {
        let file_contents = fs::read_to_string(file).expect("Failed to read file");
        output_data.push(file_contents.to_string());
    }

    let mut formatted_output_data: Vec<String> = Vec::new(); // formatted content for the json file (correct syntax)

    // formatting the content to json format
    formatted_output_data.push(concat!("{\n", r#"  "problems": ["#).to_owned()); // first line of json
    for problem in &output_data {
        let index = &output_data.iter().position(|r| r == problem).unwrap() + 1; // id of the problem

        // additional rank key and value in json file
        let rank: &str;
        match &problem {
            p if p.contains("BR[20k]WR[20k]") => rank = "20k",
            p if p.contains("BR[19k]WR[19k]") => rank = "19k",
            p if p.contains("BR[18k]WR[18k]") => rank = "18k",
            p if p.contains("BR[17k]WR[17k]") => rank = "17k",
            p if p.contains("BR[16k]WR[16k]") => rank = "16k",
            p if p.contains("BR[15k]WR[15k]") => rank = "15k",
            p if p.contains("BR[14k]WR[14k]") => rank = "14k",
            p if p.contains("BR[13k]WR[13k]") => rank = "13k",
            p if p.contains("BR[12k]WR[12k]") => rank = "12k",
            p if p.contains("BR[11k]WR[11k]") => rank = "11k",
            p if p.contains("BR[10k]WR[10k]") => rank = "10k",
            p if p.contains("BR[9k]WR[9k]") => rank = "9k",
            p if p.contains("BR[8k]WR[8k]") => rank = "8k",
            p if p.contains("BR[7k]WR[7k]") => rank = "7k",
            p if p.contains("BR[6k]WR[6k]") => rank = "6k",
            p if p.contains("BR[5k]WR[5k]") => rank = "5k",
            p if p.contains("BR[4k]WR[4k]") => rank = "4k",
            p if p.contains("BR[3k]WR[3k]") => rank = "3k",
            p if p.contains("BR[2k]WR[2k]") => rank = "2k",
            p if p.contains("BR[1k]WR[1k]") => rank = "1k",
            p if p.contains("BR[1d]WR[1d]") => rank = "1d",
            p if p.contains("BR[2d]WR[2d]") => rank = "2d",
            p if p.contains("BR[3d]WR[3d]") => rank = "3d",
            p if p.contains("BR[4d]WR[4d]") => rank = "4d",
            p if p.contains("BR[5d]WR[5d]") => rank = "5d",
            _ => {
                println!("Invalid SGF rank data. Please make sure the file contains the substring 'BR[...]WR[...]' with equal values for BR and WR between '20k' and '5d'");
                panic!();
            }
        }

        let mut obj = concat!("    {\n", r#"      "id": "#).to_string();
        obj.push_str(&index.to_string());
        obj.push_str(",\n");
        obj.push_str(r#"      "rank": ""#);
        obj.push_str(rank);
        obj.push_str(r#"""#);
        obj.push_str(",\n");
        obj.push_str(r#"      "problem": ""#);
        obj.push_str(&problem.replace('\n', "").replace('\r', ""));
        obj.push_str(r#"""#);
        obj.push_str("\n    },");
        formatted_output_data.push(obj.to_owned());
    }
    formatted_output_data.push("  ]\n}".to_owned()); // last line of json

    let formatted_output_string = formatted_output_data // final json string
        .join("\n")
        .replace("    },\n  ]", "    }\n  ]");

    fs::write("./data/problems.json", formatted_output_string).expect("Unable to write file");
}
