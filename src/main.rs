use chrono::{DateTime, TimeZone, Utc, Duration};
use rand::Rng;
use std::io;
use std::io::*;
use float_cmp::*;
use std::convert::TryInto;

fn main(){
    println!("load-planning");

    let mut plan = LoadPlan { vehicles: Vec::new() };

    loop {
        print!(">>> ");
        let _ = stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        //println!("{}",input);
        let _ = stdout().flush();
        if input.trim() == "Q" {break};
        match input.trim() {
            "L" => { list_vehicles(&plan); },
            "A" => { 
                let mut v = Vehicle::new(VehicleKind::Train);
                v.capacity = 100.0;
                v.items.push(45.5);
                plan.vehicles.push(v);
            }
            _ => { }
        }
    }
    // let mut v = Vehicle::new(VehicleKind::Train);
    // v.capacity = 100.0;
    // v.items.push(45.5);
    // assert_eq!(v.space(), 54.5);
    // v.items.push(33.3);
    // v.items.push(21.2);
    // assert_eq!(v.space(), 0.0);
    // let t1 = Utc.ymd(2020, 9, 2).and_hms(6, 0, 0);
    // let v = v.travel(t1, 41);

    // println!("{:?}", v);

    // let mut v2 = v.clone();
    // v2.items.push(3.3);
    // println!("space = {}", v2.space());
    // println!("{:?}", v);
    // println!("{:?}", v2);


    // for _ in 0..40 {
    //     v2 = v.clone();
    //     let index = (rand::random::<f32>() * v2.items.len() as f32).floor() as usize;
    //     let item = v2.items.remove(index);
    //     println!("{:?}", v2);
    //     }

}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
enum VehicleKind {
    Truck,
    Train
}

#[derive(Debug, Clone)]
struct Vehicle {
    kind: VehicleKind,
    capacity: f64,
    items: Vec<f64>,
    depart_date: DateTime<Utc>,
    arrive_date: DateTime<Utc>,
}
impl Vehicle {
    fn new(kind: VehicleKind) -> Vehicle {
        Vehicle {
            items: Vec::new(),
            kind,
            capacity: 0.0,
            depart_date: Utc.timestamp(0,0),
            arrive_date: Utc.timestamp(0,0),
        }
    }
    fn space(&self) -> f64 {
        self.capacity - self.items.iter().sum::<f64>()
    }
    fn travel(mut self, leaving: DateTime<Utc>, hours: u32) -> Vehicle {
        self.depart_date = leaving;
        self.arrive_date = leaving + Duration::hours(hours.try_into().unwrap());
        self
    }
}
#[derive(Debug,Clone)]
struct LoadPlan {
    vehicles: Vec<Vehicle>,
}

fn list_vehicles(plan: &LoadPlan) {
    for v in &plan.vehicles {
        println!("{:?}", v);
    }
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
    assert!( approx_eq!(f64, 10.1, v.space(), ulps = 3) );
}

#[test]
fn test_Travel() {
    let mut v = Vehicle::new(VehicleKind::Truck);
    assert!(v.kind == VehicleKind::Truck);
    v.capacity = 100.0;
    let start = Utc.ymd(2020, 10, 1).and_hms(6, 0, 0);
    let finish = Utc.ymd(2020, 10, 3).and_hms(7, 0, 0);
    v = v.travel(start, 49);
    assert_eq!(finish, v.arrive_date);
}