use exper_borrowed_vs_owned_parameters::{
    invoke_message_borrowed, invoke_message_default, invoke_message_owned,
    invoke_messagemf_borrowed, invoke_messagemf_default, invoke_messagemf_owned,
};

fn main() {
    invoke_message_default();
    invoke_messagemf_default();
    invoke_message_borrowed();
    invoke_message_owned();
    invoke_messagemf_borrowed();
    invoke_messagemf_owned();
}
