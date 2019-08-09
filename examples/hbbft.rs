use hbbft::dynamic_honey_badger::DynamicHoneyBadger;
use hbbft::dynamic_honey_badger::Message;
use rand::thread_rng;

pub fn loop_fn(hbd: &mut DynamicHoneyBadger<Vec<u8>,usize>, target: Message<usize>) {
    println!("--- Input loop_fn ---");
    println!("{:?}", target);
    let mut rng = thread_rng();
    let steps = hbd.handle_message(&123, target, &mut rng).unwrap();
    println!("--- Error ---: {:?}", steps.fault_log);
    for x in steps.messages.iter() {
        loop_fn(hbd, x.message.clone());
    }
    println!("--- proposed {:?} ---", hbd.should_propose());
    println!("--- next_epoch {:?} ---", hbd.next_epoch());
}

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
    println!("Result: {:?}", step.output);

    println!("fault_log is: {:?}", fault_log);
    for x in messages.iter() {
        loop_fn(&mut hbd, x.message.clone());
    }

    let step = hbd.propose(vec!(3,2,1,0),&mut rng2).unwrap();
    println!("Result: {:?}", step.output);
    for x in step.messages.iter() {
        loop_fn(&mut hbd, x.message.clone());
    }
}
