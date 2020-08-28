use chrono::{DateTime, Duration, TimeZone, Utc};
use float_cmp::*;
use im::*;
use rand::Rng;
use std::convert::TryInto;
use std::io;
use std::io::*;
use std::mem;

fn main() {
    println!("load-planning");

    // Create problem
    let mut problem = Problem::new();
    for ix in 0..5 {
        let mut order = Order::new(ix.to_string(), Utc.ymd(2020, 10, 1).and_hms(0, 0, 0));

        for _ in 0..3 {
            let f: f64 = rand::random::<f64>() * 1000.0;
            let i = f.floor() as i32;
            order.boxes.push_back(i as f64 / 10.0);
        }

        problem.orders.insert(ix.to_string(), order);
    }

    println!("{:?}", problem);

    // Create empty Plan
    let mut plan = LoadPlan::new();

    let mut current_vehicle: Option<usize> = None;
    // REPL
    loop {
        // if there is a current Vehicle selected, display it
        match current_vehicle {
            Some(i) => {
                if i <= plan.vehicles.len() {
                    print_vehicle(i, &plan.vehicles[i]);
                }
            }
            None => {}
        }
        // prompt
        print!(">>> ");
        let _ = stdout().flush();

        // get input line (command)
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        let _ = stdout().flush();

        // break command into tokens
        let cmdline: Vec<&str> = input.split_whitespace().collect();
        if cmdline.len() == 0 {
            // if no tokens, then re-prompt
            continue;
        }

        // cmd is first token
        let cmd = cmdline[0].to_uppercase();

        if cmd == "Q" {
            break;
        };
        match &cmd[..] {
            "CLEAR" => {
                // Clear Plan
                plan = LoadPlan::new();
                current_vehicle = None;
            }
            "L" => {
                // List Vehicles
                list_vehicles(&plan);
            }
            "TRAIN" => {
                // Add a Train
                let mut v = Vehicle::new(VehicleKind::Train);
                if cmdline.len() >= 2 {
                    v.capacity = parse_float(cmdline[1]);
                }
                plan.vehicles.push(v);
            }
            "TRUCK" => {
                // Add a Truck
                let mut v = Vehicle::new(VehicleKind::Truck);
                if cmdline.len() >= 2 {
                    v.capacity = parse_float(cmdline[1]);
                }
                plan.vehicles.push(v);
            }
            "SELECT" => {
                // Select a Vehicle
                if cmdline.len() >= 2 {
                    match cmdline[1].parse::<usize>() {
                        Ok(i) => {
                            if i >= plan.vehicles.len() {
                                continue;
                            }
                            current_vehicle = Some(i);
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            continue;
                        }
                    }
                }
            }
            // "LOAD" => {
            //     let mut b = problem.boxes.clone();
            //     let mut lp = LoadPlan::new();
            //     lp.vehicles = plan.vehicles.clone();
            //     loop {
            //         let mut vehicles = lp.vehicles.clone();
            //         //let mut lp = LoadPlan::new();
            //         for v in &vehicles {
            //             let b1 = b.pop();
            //             if b1 == None {
            //                 break;
            //             }
            //             let mut v1 = v.clone();
            //             v1.items.push(b1.unwrap());
            //             lp.vehicles.push(v1);
            //         }
            //         if b.len() == 0 {
            //             break;
            //         }
            //     }
            //     plan = lp;
            // }
            "T" => {
                let mut lp = plan.clone();
                lp.vehicles[0].items.push_back(("1".to_string(), 99.0));
                plan = lp.clone();
            }
            _ => {
                // Unknown command
                println!("Sorry...");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VehicleKind {
    Truck,
    Train,
}

#[derive(Debug, Clone)]
struct Problem {
    orders: HashMap<String, Order>,
}
impl Problem {
    fn new() -> Self {
        Self {
            orders: hashmap! {},
        }
    }
}

#[derive(Debug, Clone)]
struct Order {
    id: String,
    boxes: Vector<f64>, // weights
    due_date: DateTime<Utc>,
}
impl Order {
    fn new(id: String, due_date: DateTime<Utc>) -> Self {
        Self {
            boxes: Vector::<f64>::new(),
            id,
            due_date,
        }
    }
}

#[derive(Debug, Clone)]
struct Vehicle {
    kind: VehicleKind,
    capacity: f64,
    items: Vector<(String, f64)>, // order id, box size
    depart_date: DateTime<Utc>,
    arrive_date: DateTime<Utc>,
}
impl Vehicle {
    fn new(kind: VehicleKind) -> Vehicle {
        Vehicle {
            items: Vector::new(),
            kind,
            capacity: 0.0,
            depart_date: Utc.timestamp(0, 0),
            arrive_date: Utc.timestamp(0, 0),
        }
    }
    fn space(&self) -> f64 {
        self.capacity - self.items.iter().map(|x| x.1).sum::<f64>()
    }
    fn travel(mut self, leaving: DateTime<Utc>, hours: u32) -> Vehicle {
        self.depart_date = leaving;
        self.arrive_date = leaving + Duration::hours(hours.try_into().unwrap());
        self
    }
}
#[derive(Debug, Clone)]
struct LoadPlan {
    vehicles: Vec<Vehicle>,
}
impl LoadPlan {
    fn new() -> Self {
        Self {
            vehicles: Vec::new(),
        }
    }
}
// return zero if bad value
fn parse_float(s: &str) -> f64 {
    match s.parse() {
        Ok(val) => val,
        Err(_) => 0.0,
    }
}
fn list_vehicles(plan: &LoadPlan) {
    for (i, v) in plan.vehicles.iter().enumerate() {
        print_vehicle(i, v);
    }
    println!();
}
fn print_vehicle(i: usize, v: &Vehicle) {
    println!("{:4}: {:?}", i, v);
}

// tests

#[test]
fn test_new_vehicle() {
    let v = Vehicle::new(VehicleKind::Truck);
    assert!(v.kind == VehicleKind::Truck);
    let v = Vehicle::new(VehicleKind::Train);
    assert!(v.kind == VehicleKind::Train);
}

#[test]
fn test_space() {
    let mut v = Vehicle::new(VehicleKind::Truck);
    assert!(v.kind == VehicleKind::Truck);
    v.capacity = 100.0;
    v.items.push(77.0);
    assert_eq!(23.0, v.space());
    v.items.push(12.9);
    assert!(approx_eq!(f64, 10.1, v.space(), ulps = 3));
}

#[test]
fn test_travel() {
    let mut v = Vehicle::new(VehicleKind::Truck);
    assert!(v.kind == VehicleKind::Truck);
    v.capacity = 100.0;
    let start = Utc.ymd(2020, 10, 1).and_hms(6, 0, 0);
    let finish = Utc.ymd(2020, 10, 3).and_hms(7, 0, 0);
    v = v.travel(start, 49);
    assert_eq!(finish, v.arrive_date);
}

#[test]
fn test_parse_float() {
    assert_eq!(0.0, parse_float("xyz"));
    assert_eq!(0.0, parse_float(""));
    assert_eq!(123.456, parse_float("123.456"));
}
