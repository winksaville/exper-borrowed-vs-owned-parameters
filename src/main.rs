use exper_borrowed_vs_owned_parameters::{
    invoke_message_borrowed, invoke_message_borrowed_idx_loop, invoke_message_borrowed_iter_loop,
    invoke_message_owned, invoke_message_owned_idx_loop, invoke_message_owned_iter_loop,
};

fn main() {
    invoke_message_borrowed();
    invoke_message_borrowed_idx_loop();
    invoke_message_borrowed_iter_loop();
    invoke_message_owned();
    invoke_message_owned_idx_loop();
    invoke_message_owned_iter_loop();
}
