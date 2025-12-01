use shards::{build_shards, connect_shards, make_shard_process, process::{self, ProcessHandle}};



fn main() {
    make_shard_process!(
       AddProcess,
       |x: u32, y: u32| -> u32 {
           x + y
       },
       inputs {
           x: u32,
           y: u32,
       },
       outputs {
           val: u32,
       }
    );
    let input_value = 5;
    make_shard_process!(
        InputProcess,
        move || -> u32 {
            input_value
        },
        inputs {},
        outputs {
            val: u32,
        }
    );
    make_shard_process!(
        OutputProcess,
        |res: u32| -> () {println!("res: {}", res);},
        inputs {
            res: u32
        },
        outputs {}
    );

    
    let add = AddProcess::builder();
    let input_x = InputProcess::builder();
    let input_y = InputProcess::builder();
    let output = OutputProcess::builder();

    connect_shards!(
        (input_x.val => add.x),
        (input_y.val => add.y),
        (add.sum => output.res),
    );

    let handles: Vec<ProcessHandle> = vec![
        add.build().run(),
        input_x.build().run(),
        input_y.build().run(),
        output.build().run(),
    ];

    for handle in handles {
        let result = handle.join();
        println!("Process exited with result {:?}", result);
    }

}
