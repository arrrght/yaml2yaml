use clap::{values_t, value_t, App};
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Opt {
    config: String,
    files: Vec<String>,
    is_overwrite: bool,
    is_ignore: bool,
    no_backup: bool,
}

#[derive(Debug)]
struct Do {
    config: Vec<Yaml>,
    wk: HashMap<String, Vec<Yaml>>
}

impl Do {
    fn roulette(&mut self, v: String) -> Yaml {
        let arr = self.wk.get_mut(&v).unwrap();
        arr.reverse();
        let some = arr.pop().unwrap();
        arr.reverse();
        arr.push(some.clone());
        //arr.reverse();
        some
    }
    fn init(config_file_name: &str) -> Do {
        let f_cnf = read_to_string(config_file_name).unwrap();
        let cnf = YamlLoader::load_from_str(&f_cnf).unwrap();
        let doc0 = cnf[0]["default"].as_vec().unwrap().to_vec();

        let mut wk: HashMap<String, Vec<Yaml>> = HashMap::new();
        for i in doc0.clone() {
            let hash = i.as_hash().unwrap();
            let name = hash.get(&Yaml::from_str("name")).unwrap().as_str().unwrap();
            let v = hash.get(&Yaml::from_str("to")).unwrap().as_vec().unwrap().to_vec();
            wk.insert(name.to_owned(), v.clone());
        }

        Do {
            config: doc0,
            wk: wk
        }
    }
}

fn main() {
    let matches = App::new("yaml2yaml")
        .about(clap::crate_description!())
        .arg_from_usage("-c, --config[config.yml] 'Use other config file'")
        .arg_from_usage("-w, --overwrite 'Overwrite original'")
        .arg_from_usage("-n, --no-backup 'Disable backup to *.backup'")
        .arg_from_usage("-i, --ignore 'Ignore not existent files'")
        .arg_from_usage("<filename>... 'Files to convert'")
        .get_matches();

    //println!("FNS: {:?}", values_t!(matches, "filename", String).unwrap());

    let opt = Opt {
        config: value_t!(matches, "config", String).unwrap_or("config.yml".to_string()),
        files: values_t!(matches, "filename", String).unwrap(),
        is_ignore: match matches.is_present("ignore") {
            true => true,
            _ => false,
        },
        is_overwrite: match matches.is_present("overwrite") {
            true => true,
            _ => false,
        },
        no_backup: match matches.is_present("no-backup") {
            true => true,
            _ => false,
        },

    };

    let mut config = Do::init(&opt.config);

    for filename in opt.files {

        if !std::path::Path::new(&filename).exists() {
            if opt.is_ignore {
                println!("File {} not exists. Ignored.", filename);
            }else{
                println!("File {} is not exists. Program exit.", filename);
                std::process::exit(1);
            }
        }else{
            let f_str = read_to_string(filename.clone()).unwrap();
            let docker_config = YamlLoader::load_from_str(&f_str).unwrap();
            let mut docker_c = docker_config[0].clone();


            walk_node(&mut docker_c, Vec::new(), &mut config);

            let mut out_str = String::new();
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(&docker_c).unwrap();
            match opt.is_overwrite {
                true => {
                    if !opt.no_backup {
                        std::fs::rename(filename.clone(), format!("{}.backup", filename.clone())).expect("Can't backup");
                    }
                    let mut file = File::create(filename.clone()).unwrap();
                    file.write_all(&out_str.as_bytes()).unwrap();
                },
                false => {
                    let mut file = File::create("out.yml").unwrap();
                    file.write_all(&out_str.as_bytes()).unwrap();
                }
            }
            println!("{} converted", filename);
        }
    }
}

fn compare_arr(cmp1: &Vec<String>, cmp2: &Vec<&str>) -> bool {
    if cmp1.len() != cmp2.len() {
        return false;
    }
    let (mut i1, mut i2) = (cmp1.into_iter(), cmp2.into_iter());
    while let (Some(v1), Some(v2)) = (i1.next(), i2.next()) {
        match v2 {
            &"*" => (),
            _ if (v1 != v2) => return false,
            _ => (),
        }
    }
    true
}
fn compare_node(path: Vec<String>, config: &mut Do) -> Option<Yaml> {
    for ref mut item in &config.config {
        let h_name = &item.as_hash().unwrap().clone();
        let s_name:String = h_name.get(&Yaml::String("name".to_string())).unwrap().as_str().unwrap().to_string();
        let s_arr: Vec<&str> = s_name.split("/").collect();
        if compare_arr(&path, &s_arr) {
            let val = config.roulette(s_name);
            let method = h_name.get(&Yaml::String("method".to_string())).unwrap().clone();

            if method.as_str() == Some("roulette") {
                let part = h_name.get(&Yaml::from_str("part")).unwrap().as_str().unwrap();
                let mut h = val.as_hash().unwrap().clone();
                let first = h.entries().next().unwrap();
                let ret = match part {
                    "key" => first.key(),
                    _ => first.get()
                };
                return Some(ret.clone());
            }
            return Some(val.clone());
        }
    }
    None
}

fn walk_node(doc: &mut yaml::Yaml, path: Vec<String>, config: &mut Do) {
    match doc {
        Yaml::Array(ref mut v) => {
            for x in v {
                let mut path = path.clone();
                match x.clone() {
                    Yaml::String(s) => path.push(s.to_string()),
                    Yaml::Integer(i) => path.push(i.to_string()),
                    Yaml::Hash(_) => path.push("#".to_string()),
                    some => println!("F>U>C>K {:?}",some)
                }
                walk_node(&mut *x, path, config);
            }
        }
        Yaml::Hash(ref mut h) => {
            for (k, v) in h.iter_mut() {
                let mut path2 = path.clone();
                let last = k.to_owned().into_string().unwrap();
                path2.push(last);
                match compare_node(path2, config) {
                    Some(x) => *v = x,
                    None => ()
                }
                let mut path = path.clone();
                path.push(k.clone().into_string().unwrap());
                walk_node(v, path, config);
            }
        }
        _ => {}
    }
}
