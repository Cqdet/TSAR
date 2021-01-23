use std::fs::*;
use std::io::*;
use std::path::*;
use std::env::*;

fn main() {
    let args: Vec<String> = args().collect::<Vec<String>>();
    let (cmd, item) = (&args[1], &args[2]); 
    if cmd == "bundle" {
        if Some(item) == None {
            panic!("Pass in an item to bundle");
        }

        bundle(String::from(item));
    } else if cmd == "unbundle" {
        if Some(item) == None {
            panic!("Pass in a directory to unbundle");
        }

        unbundle(String::from(item));
    } else {
        panic!("Invalid argument!");
    }
}

pub fn bundle(dir: String) {
    append_out_file(None);
    read_recursive(dir, Vec::new());
}

pub fn unbundle(file: String) {
    let data = read_to_string(file).expect("FILE ERROR: Unable to read from file");
    for line in data.split("\n") {
        if line.len() < 1 { continue };
        let split = line.split(":").collect::<Vec<&str>>();
        let (name, content) = (split[0], split[1]);
        if name.chars().nth(0).unwrap() == '.' { continue }; // Ignores hidden files
        let mut path_arr = name.split("/").collect::<Vec<&str>>();
        &path_arr.remove(path_arr.len() - 1);
        let full_path = &format!("out/{}", path_arr.join("/"));
        if !Path::new(&full_path).exists() {
            create_dir_all(full_path).expect("FILE ERROR: Unable to create directory");
        }

        let uint8_data: Vec<u8> = content.split(",").collect::<Vec<&str>>().iter().map(|x| {
            x.parse::<u8>().unwrap()
        }).collect();
        write(format!("out/{}", name), uint8_data).expect("FILE ERROR: Unable to create/write file");
    }
}   


fn append_out_file(data: Option<String>) {
    if data == None {
        File::create("out.tsar").expect("FILE ERROR: Unable to create file");
    } else {
        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("out.tsar")
        .unwrap();
        writeln!(file, "{}", data.unwrap()).expect("FILE ERROR: Unable to write to file");
    }   
}

fn read_recursive(path: String, mut arr_files: Vec<String>) -> Vec<String> {
    let paths = read_dir(&path).unwrap();

    for item in paths {
        let name = &item.unwrap().file_name().into_string().unwrap();
        if name.chars().nth(0).unwrap() == '.' { continue } // Ignores hidden files
        if metadata(format!("{}/{}", path, name)).unwrap().is_dir() {
            arr_files = read_recursive(format!("{}/{}", path, name), arr_files);
        } else {
            let full_path = &format!("{}/{}", path, name);
            let mut file_content: Vec<u8> = Vec::new();
            
            File::open(full_path)
            .expect("FILE ERROR: Unable to read from file")
            .read_to_end(&mut file_content)
            .iter()
            .map(|x| 
                {
                    x.to_string()
                }
            ).collect::<Vec<String>>()
            .join(",");

            append_out_file(Some(format!("{}:{}", full_path, file_content
                .iter()
                .map(|x| {
                    x.to_string()
                })
                .collect::<Vec<String>>()
                .join(","))
            ));
        }
    }
    
    return arr_files;

}
