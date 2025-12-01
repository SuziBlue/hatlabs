
#[macro_export]
macro_rules! connect_shards {
    (
        $(
            ($($conn:tt)+)
        ),* $(,)?
    ) => {
        $(
            $crate::connect_shards!(@expand $($conn)+);
        )*
    };

    // Full proc.proc => proc.proc
    (@expand $($sender_proc:ident . $sender_field:ident),+ => $($receiver_proc:ident . $receiver_field:ident),+) => {
        let (tx, rx) = crossbeam_channel::unbounded();
        $(
            let $sender_proc = $sender_proc.$sender_field(tx.clone());
        )+
        $(
            let $receiver_proc = $receiver_proc.$receiver_field(rx.clone());
        )+
        drop(tx);
        drop(rx);
    };

    // Proc.field => receiver var: Type
    (@expand $sender_proc:ident . $sender_field:ident => $receiver_var:ident : $ty:ty) => {
        let (tx, rx): (crossbeam_channel::Sender<$ty>, crossbeam_channel::Receiver<$ty>) = crossbeam_channel::unbounded();
        let $sender_proc = $sender_proc.$sender_field(tx);
        let $receiver_var = rx;
    };

    // Var: Type => proc.field
    (@expand $sender_var:ident : $ty:ty => $receiver_proc:ident . $receiver_field:ident) => {
        let (tx, rx): (crossbeam_channel::Sender<$ty>, crossbeam_channel::Receiver<$ty>) = crossbeam_channel::unbounded();
        let $receiver_proc = $receiver_proc.$receiver_field(rx);
        let $sender_var = tx;
    };
}

