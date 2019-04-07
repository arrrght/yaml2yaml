Change vars in one Yaml file, according to rules in second Yaml(v1.2) file

For example, change some vals in docker-compose.yml (Rancher) by rules

```
accounts:
  _worked: &worked
    - user1: pass1
    - user2: pass2

  _non-worked: &non_worked
    - user3: pass3 
    - user4: pass4 

  default: *template1

templates:
  template1: &template1
    - { name: "/services/*/environment/login_mindoo_mileage", to: *worked, method: roulette, part: 0 }
    - { name: "/services/*/environment/password_mindoo_mileage", to: *worked, method: random, part: 1 }
    - { name: var, to: val, method: single }
    - { name: some, to: *worked, method: roulette }

  default: *template1
```
