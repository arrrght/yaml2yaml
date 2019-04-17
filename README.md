Change vars in one Yaml file, according to rules in second Yaml(v1.2) file

For example, change some vals in docker-compose.yml (Rancher) by rules

```
accounts:
  _worked: &worked
    - user1: pass1
    - user2: pass2
    - user3: pass3
    - user4: pass4
    - user5: pass5

  _non-worked: &non_worked
    - user3: pass3 
    - user4: pass4 

templates:
  template1: &template1
    - { name: "services/*/environment/login_mindoo_mileage", to: *worked, method: roulette, part: key }
    - { name: "services/*/environment/password_mindoo_mileage", to: *worked, method: roulette, part: val }

default: *template1
```

## Usage

```
yaml2yaml 
Convert yaml file, based on rules in config.yaml

USAGE:
    yaml2yaml [FLAGS] [OPTIONS] <filename>...

FLAGS:
    -h, --help         Prints help information
    -i, --ignore       Ignore not existent files
    -n, --no-backup    Disable backup to *.backup
    -w, --overwrite    Overwrite original
    -V, --version      Prints version information

OPTIONS:
    -c, --config <config.yml>    Use other config file

ARGS:
    <filename>...    Files to convert
```
