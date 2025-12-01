use shards::{connect_shards, make_shard_process};



fn main() {

    make_shard_process!(
        InputA,
        |a: u32| -> (val: u32) {
            a
        }
    );
    make_shard_process!(
        InputB,
        |b: u32| -> (val: u32) {
            b
        }
    );
    make_shard_process!(
        Output,
        |val: u32| -> (out: u32) {
            val
        }
    );

    let input_a = InputA::builder();
    let input_b = InputB::builder();
    let output = Output::builder();

    connect_shards!(
        (input_a.val, input_b.val => output.val),
        (tx_a: u32 => input_a.a),
        (tx_b: u32 => input_b.b),
        (output.out => rx_out: u32),
    );

    let input_a = input_a.build();
    let input_b = input_b.build();
    let output = output.build();

    let h1 = input_a.run();
    let h2 = input_b.run();
    let h3 = output.run();


    let a = 5;
    println!("Sending {}", a);
    let _ = tx_a.send(a);

    assert_eq!(rx_out.recv().unwrap(),a);
    println!("Received {}", a);

    let b = 2;
    println!("Sending {}", b);
    let _ = tx_b.send(b);

    assert_eq!(rx_out.recv().unwrap(),b);
    println!("Received {}", b);

    drop(tx_a);
    drop(tx_b);
    drop(rx_out);

    let _ = h1.join();
    let _ = h2.join();
    let _ = h3.join();
    
}
