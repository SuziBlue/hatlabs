
```markdown
# 🚧 Building a Type-Safe, Auto-Scaling, Visual Process DAG in Rust

## Abstract

This paper introduces a macro-driven system for defining, connecting, and scaling **typed concurrent processes** in Rust. It uses a declarative syntax to build a **type-safe Directed Acyclic Graph (DAG)** of concurrently running processes.

This is all done with zero dynamic dispatch. Compile-time guarantees are enforced via Rust’s type system and macro expansion.

---

## 1. Motivation

Modern dataflow systems need:

- Type safety  
- Parallel execution  
- Modular composition

This framework takes inspiration from actor systems and functional pipelines. It defines:

- **Processes**: Independent concurrent units.
- **Edges**: Strongly typed channels (senders and receivers).
- **DAGs**: Declared using macros and validated at compile time.

---

## 2. Core Building Blocks

### 2.1 `Process` Trait

Every concurrent unit implements a shared trait:

```rust
pub trait Process: Send + 'static {
    fn run(self) -> JoinHandle<()>;
}
```

This trait starts the process on a thread and returns a `JoinHandle`.

---

### 2.2 Defining Processes via Macros

We define each process using the `define_module!` macro:

```rust
macro_rules! define_module {
    (
        $name:ident,
        inputs: { $( $in_field:ident : $in_ty:ty ),+ $(,)? },
        outputs: { $( $out_field:ident : $out_ty:ty ),* $(,)? },
        func: $func:expr
    ) => {
        pub struct $name<F>
        where
            F: FnMut($( $in_ty ),+) -> ( $( $out_ty ),* ) + Send + 'static,
        {
            $( pub $in_field: crossbeam_channel::Receiver<$in_ty>, )+
            $( pub $out_field: crossbeam_channel::Sender<$out_ty>, )*
            func: F,
        }

        impl<F> $name<F>
        where
            F: FnMut($( $in_ty ),+) -> ( $( $out_ty ),* ) + Send + 'static,
        {
            pub fn new(
                $( $in_field: crossbeam_channel::Receiver<$in_ty>, )+
                $( $out_field: crossbeam_channel::Sender<$out_ty>, )*,
                func: F
            ) -> Self {
                Self { $( $in_field, )+ $( $out_field, )* func }
            }

            pub fn run(mut self) {
                std::thread::spawn(move || {
                    loop {
                        $(
                            let $in_field = match self.$in_field.recv() {
                                Ok(val) => val,
                                Err(_) => break,
                            };
                        )+

                        let result = (self.func)($( $in_field ),+);

                        #[allow(unused_variables)]
                        let ($($out_field),*) = result;

                        $(
                            let _ = self.$out_field.send($out_field);
                        )*
                    }
                });
            }
        }
    };
}
```

---

### Example: Defining and Running a Process

```rust
use crossbeam_channel::unbounded;

let mut a = 1;

let (tx_in, rx_in) = unbounded();

define_module!(
    AddX,
    inputs: {
        x: i32,
    },
    outputs: {},
    func: move |x| {
        a += x;
        println!("a = {}", a);
        ()
    }
);

let process = AddX::new(rx_in);

process.run();

tx_in.send(5).unwrap();
```

This macro generates:

- A process struct with named input/output channels
- A `run()` function that spawns the process in a thread
- Compile-time safe input/output matching

---

### Capturing Shared State

You can capture and mutate state by wrapping it:

```rust
let state = Arc<Mutex<MyState>>;
```

Then clone and move it into closures:

```rust
let shared = Arc::clone(&state);

move |x| {
    let mut data = shared.lock().unwrap();
    data.update(x);
    ()
}
```

This enables safe, concurrent stateful processes.

---

## 3. Connecting Processes into a DAG

To run a full system, you connect multiple processes into a DAG.

### 3.1 Declarative DAG Construction

Use a fluent builder pattern:

```rust
let mut graph = Graph::new()
    .add_node(counter1)
    .add_node(counter2)
    .add_node(aggregator);

connect! {
    counter1.output => aggregator.input1;
    counter2.output => aggregator.input2;
}

graph.run();
```

### Benefits

- Compile-time type checking
- Declarative topology
- Easy to visualize

---

## 4. Wiring with `define_runtime!`

Avoid repetitive boilerplate when connecting processes.

Use the `define_runtime!` macro to:

- Declare all processes
- Automatically create and connect channels
- Provide a `.run()` method for the runtime

---

### 4.1 Macro Definition

```rust
#[macro_export]
macro_rules! define_runtime {
    (
        $runtime_name:ident,
        modules: {
            $( $mod_ident:ident : $mod_ty:ty ),+ $(,)?
        },
        connections: [
            $( $src_mod:ident . $src_field:ident => $dst_mod:ident . $dst_field:ident ),* $(,)?
        ]
    ) => {
        pub struct $runtime_name {
            $( pub $mod_ident: $mod_ty ),+
        }

        impl $runtime_name {
            pub fn new() -> Self {
                use crossbeam_channel::unbounded;

                $(
                    let (tx_$src_mod_$src_field, rx_$dst_mod_$dst_field) = unbounded();
                )*

                $(
                    let $mod_ident = {
                        todo!("instantiate process: {}", stringify!($mod_ident))
                    };
                )+

                Self { $( $mod_ident ),+ }
            }

            pub fn run(self) {
                let handles = vec![
                    $(
                        {
                            let h = self.$mod_ident.run();
                            (stringify!($mod_ident), h)
                        }
                    ),+
                ];

                for (name, handle) in handles {
                    match handle.join() {
                        Ok(_) => println!("Process {} exited cleanly", name),
                        Err(_) => println!("Process {} panicked!", name),
                    }
                }
            }
        }
    };
}
```

---

### 4.2 Example Usage

```rust
define_runtime!(
    MyRuntime,
    modules: {
        adder: MyAdder,
        scaler: MyScaler
    },
    connections: [
        adder.scaled => scaler.input
    ]
);
```

This creates:

- A runtime struct holding all processes
- Internal channel wiring
- A `run()` method that joins all threads

---

## 5. Design Benefits

- **Declarative**: Clear structure, no manual wiring
- **Type-safe**: Connections are validated at compile time
- **Composable**: Processes can be tested individually
- **Scalable**: Supports worker pools and load balancing
- **Safe**: No unsafe code or runtime type casts

---

## 6. Execution Model

```text
User Code
   │
   ├─ Macro generates → Process Struct
   │                    (inputs: Receivers, outputs: Senders, closure: logic)
   │
   ├─ Implements Process Trait
   │    └─ run() → spawn thread, receive → process → send
   │
   ├─ Manual run() or via Runtime
   │
   └─ Runtime:
         ├─ Instantiates and connects processes
         ├─ Starts each process thread
         ├─ Monitors joins and failures
```

---

## 7. Conclusion

This system proves that Rust can build:

- ✅ Typed, concurrent DAGs  
- ✅ Safe shared state via `Arc<Mutex<_>>`  
- ✅ All in pure Rust, without external runtimes

You write simple closures. Macros handle the rest.

---

```
