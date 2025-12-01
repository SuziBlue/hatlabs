use std::thread::JoinHandle;

pub trait Process: Send + 'static {
    fn run(self) -> ProcessHandle;
}

#[derive(Debug)]
pub struct ProcessHandle {
    join_handle: JoinHandle<()>
}

impl ProcessHandle {
    pub fn new(join_handle: JoinHandle<()>) -> Self {
        Self { join_handle }
    }
    pub fn join(self) -> std::thread::Result<()> {
        self.join_handle.join()
    }
}



#[macro_export]
macro_rules! make_shard_process {
    (
        $process_name:ident,
        $closure:expr,
        inputs { $( $in_name:ident : $in_ty:ty ),* $(,)? },
        outputs { $( $out_name:ident : $out_ty:ty ),* $(,)? }
    ) => {
        #[derive(typed_builder::TypedBuilder)]
        pub struct $process_name {
            $(
                #[builder(setter(into))]
                pub $in_name: crossbeam_channel::Receiver<$in_ty>,
            )*
            $(
                #[builder(setter(into))]
                pub $out_name: crossbeam_channel::Sender<$out_ty>,
            )*
        }

        impl $process_name {
            pub fn run(mut self) -> $crate::process::ProcessHandle {

                let join_handle = std::thread::spawn(move || {
                    loop {
                        $(
                            let $in_name = match self.$in_name.recv() {
                                Ok(val) => val,
                                Err(_) => break,
                            };
                        )*

                        let ($($out_name),*) = $closure($($in_name),*);

                        $(
                            let _ = self.$out_name.send($out_name);
                        )*
                    }
                });

                $crate::process::ProcessHandle::new(join_handle)
            }
        }
    };
}

#[macro_export]
macro_rules! build_shards {
    (
        $( $process:ident ),+
    ) =>{
        $(
            $process = $process.build();
        )*
    };
}

#[macro_export]
macro_rules! run_shards{
    (
        $( $process:ident ),*
    ) =>{
        $(
            let $process = $process.run(); 
        )*
    };
}

/// Macro to define and spawn a process from a closure and input receivers
#[macro_export]
macro_rules! process {
    (|$($arg:ident),*| $body:expr; $(.$field:ident($rx:expr))*) => {{
        // Spawn a thread that receives arguments and executes the closure
        thread::spawn(move || {
            $(
                let $arg = $rx.recv().expect(concat!(stringify!($field), " failed to receive"));
            )*
            let result = $body;
            println!("Process result: {:?}", result);
            result
        })
    }};
}
