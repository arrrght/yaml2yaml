use clap::{value_t, App};
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};

#[derive(Debug, Clone)]
struct Opt {
    dir: String,
    config: String,
    file: String,
}

#[derive(Debug)]
struct Do {
    config: Vec<Yaml>
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

    let mut config = Do {
        config: {
            let f_cnf = read_to_string(opt.config.clone()).unwrap();
            let cnf = YamlLoader::load_from_str(&f_cnf).unwrap();
            let doc = &cnf[0];
            doc["default"].as_vec().unwrap().to_vec()
        }
    };


    let f_str = read_to_string(opt.file).unwrap();
    let docker_config = YamlLoader::load_from_str(&f_str).unwrap();
    let mut docker_c = docker_config[0].clone();

//    let f_cnf = read_to_string(opt.config).unwrap();
//    let cnf = YamlLoader::load_from_str(&f_cnf).unwrap();
//    let doc = &cnf[0];
//    let def = doc["default"].as_vec().unwrap();

    walk_node(&mut docker_c, Vec::new(), &mut config);

//    for item in def {
//        let h_name = item.as_hash().unwrap();
//        let s_name = h_name.get(&Yaml::String("name".to_string())).unwrap().as_str().unwrap();
//        //let val = h_name.get(&Yaml::String("to".to_string())).unwrap();
//        //println!("walking into {:?}::{:?}", s_name, &val);
//        let sample: Vec<&str> = s_name.split("/").collect();
//        walk_node(&mut docker_c, Vec::new(), &sample, &item);
//    }

    //let sample: Vec<&str> = "services/*/environment/cicle_history_host".split("/").collect();
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(&docker_c).unwrap();
    let mut file = File::create("out.yml").unwrap();
    file.write_all(&out_str.as_bytes()).unwrap();
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
   // println!("{:?} = {:?}", cmp1.join("/"), cmp2.join("/"));
    true
}
fn compare_node2(path: Vec<String>, config: &mut Do) -> Option<Yaml> {
    println!("H {:?}", config);
    //println!("AAA {:?}, {:?}", path, config.config);
    for ref mut item in &config.config {
        let h_name = &item.as_hash().unwrap().clone();
        let s_name:String = h_name.get(&Yaml::String("name".to_string())).unwrap().as_str().unwrap().to_string();
        let s_arr: Vec<&str> = s_name.split("/").collect();
        if compare_arr(&path, &s_arr) {
            let val = h_name.get(&Yaml::String("to".to_string())).unwrap();
            let method = h_name.get(&Yaml::String("method".to_string())).unwrap().clone();
            let ref mut to = h_name.get(&Yaml::String("to".to_string())).unwrap();
            let mut a2 = to.clone();
            let mut a3 = a2.as_vec().unwrap().clone();
            let pop = a3.pop().unwrap();
            a3.reverse();
            a3.push(pop.clone());
            a3.reverse();
            let ya3 = Yaml::Array(a3);
            //*to = &ya3;
            *item = &ya3;
            println!("H {:?}", to);

            //*item = &Yaml::String("ASDASD".to_string());
            if method.as_str() == Some("roulette") {
                //println!("M {:?}", to);
                *to = &val.clone();
                return Some(method);
            }
            //println!("method: {:?}", method);
            return Some(val.clone());
        }
    }
    None
}

fn walk_node(doc: &mut yaml::Yaml, path: Vec<String>, config: &mut Do) {
    //let sample: Vec<&str> = "services/*/environment/cicle_history_host".split("/").collect();
    match doc {
        Yaml::Array(ref mut v) => {
            for x in v {
                //compare_node(&sample, &path, x.to_owned().into_string());
                let mut path = path.clone();
                match x.clone() {
                    Yaml::String(s) => path.push(s.to_string()),
                    Yaml::Integer(i) => path.push(i.to_string()),
                    Yaml::Hash(_) => path.push("#".to_string()),
                    some => println!("F>U>C>K {:?}",some)
                }
                //path.push(x.to_string());
                walk_node(&mut *x, path, config);
            }
        }
        Yaml::Hash(ref mut h) => {
            for (k, v) in h.iter_mut() {
                let mut path2 = path.clone();
                let last = k.to_owned().into_string().unwrap();
                path2.push(last);
                match compare_node2(path2, config) {
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
