use exper_borrowed_vs_owned_parameters::{
    invoke_message_borrowed,
    //invoke_message_borrowed_for_iter,
    invoke_message_owned,
    //invoke_message_owned_for_iter,
};


fn main() {
    invoke_message_borrowed();
    //invoke_message_borrowed_for_iter();
    invoke_message_owned();
    //invoke_message_owned_for_iter();
}
