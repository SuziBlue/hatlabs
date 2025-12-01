use shards::{connect_shards, make_shard_process};




fn main() {
    make_shard_process!(
        Input,
        |input: u32| -> (val: u32) {
            input
        }
    );
    make_shard_process!(
        Output,
        |val: u32| -> (output: u32) {
            val
        }
    );


    let input = Input::builder();
    let output = Output::builder();

    connect_shards!(
        (sender: u32 => input.input),
        (input.val => output.val),
        (output.output => receiver: u32),
    );

    let input = input.build();
    let output = output.build();

    let h1 = input.run();
    let h2 = output.run();

    let inputs = vec![1,2,3,4];
    for value in inputs.clone() {
        let _ = sender.send(value);
    }
    drop(sender);
    let outputs: Vec<u32> = receiver.iter().collect();
    

    assert_eq!(inputs, outputs);
    println!("Inputs: {:?}, Outputs: {:?}", inputs, outputs);

    let _ = h1.join();
    let _ = h2.join();
}
