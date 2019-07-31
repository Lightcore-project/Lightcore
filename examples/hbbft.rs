use hbbft::dynamic_honey_badger::DynamicHoneyBadger;
use rand::thread_rng;

pub fn main() {
    let mut rng1 = thread_rng();
    let mut hbd = DynamicHoneyBadger::<Vec<u8>,usize>::builder().build_first_node(123,&mut rng1).unwrap();
    println!("{:?}",hbd.next_epoch());
    println!("{:?}",hbd.netinfo());
    println!("{:?}",hbd.has_input());
    println!("{:?}",hbd.should_propose());
    let mut rng2 = thread_rng();
    let step = hbd.propose(vec!(0,1,2,3),&mut rng2).unwrap();
    let fault_log = step.fault_log;
    let messages = step.messages;

    println!("{:?}", fault_log);
    for x in messages.iter() {
        println!("{:?}", x);
    }
}
