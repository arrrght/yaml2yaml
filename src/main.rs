use clap::{value_t, App};
use linked_hash_map::LinkedHashMap;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};

#[derive(Debug, Clone)]
struct Opt {
    dir: String,
    config: String,
    file: String,
}

fn main() {
    let matches = App::new("some")
        .arg_from_usage("-c, --config[config.yml] 'Use other config file'")
        .arg_from_usage("-d, --dir[docker] 'Use directory'")
        .arg_from_usage("-f, --file[docker-compose.yml] 'Use that file as template, 4TEST'")
        .get_matches();

    let opt = Opt {
        dir: value_t!(matches, "dir", String).unwrap_or("docker".to_string()),
        config: value_t!(matches, "config", String).unwrap_or("config.yml".to_string()),
        file: value_t!(matches, "file", String).unwrap_or("./docker-compose.yml".to_string()),
    };

    let f_str = read_to_string(opt.file).unwrap();

    let docker_config = YamlLoader::load_from_str(&f_str).unwrap();
    let mut docker_c = docker_config[0].clone();
    //let mut doc = docker_config[0].as_hash().unwrap().clone();

    // worked
    //*doc.get_mut(&Yaml::String("version".to_string())).unwrap() = Yaml::String("ASASDASD".to_string());

    walk_node(&mut docker_c, Vec::new());
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(&docker_c).unwrap();
    let mut file = File::create("out.yml").unwrap();
    file.write_all(&out_str.as_bytes()).unwrap();
}

fn compare_node(cmp1: &Vec<&str>, cmp2: &Vec<String>, now: Option<String>) -> bool {
    let mut cmp2 = cmp2.to_owned();
    match now {
        Some(x) => {
            //println!("{:?}:{:?} + {:?}", cmp1.join("/"), cmp2.join("/"), x); // DEBUG
            cmp2.push(x)
        }
        _ => (()),
    };
    let cmp2 = &cmp2;
    if cmp1.len() != cmp2.len() {
        return false;
    }
    let (mut i1, mut i2) = (cmp1.into_iter(), cmp2.into_iter());
    while let (Some(v1), Some(v2)) = (i1.next(), i2.next()) {
        match v1 {
            &"*" => (),
            _ if (v1 != v2) => return false,
            _ => (),
        }
    }
    println!("{:?} = {:?}", cmp1.join("/"), cmp2.join("/"));
    true
}

fn walk_node(doc: &mut yaml::Yaml, path: Vec<String>) {
    let sample: Vec<&str> = "services/*/environment/cicle_history_host".split("/").collect();
    match doc {
        yaml::Yaml::Array(ref mut v) => {
            for x in v {
                //compare_node(&sample, &path, x.to_owned().into_string());
                let mut path = path.clone();
                path.push(x.clone().into_string().unwrap());
                walk_node(&mut *x, path);
            }
        }
        yaml::Yaml::Hash(ref mut h) => {
            for (k, v) in h.iter_mut() {
                match compare_node(&sample, &path, k.to_owned().into_string()) {
                    true => {
                        //println!("IN: {:?} - {:?}", k, v);
                        *v = Yaml::String("YESLIUKANG".to_string());
                    }
                    false => (),
                }
                let mut path = path.clone();
                path.push(k.clone().into_string().unwrap());
                walk_node(v, path);
            }
        }
        _ => {}
    }
}
