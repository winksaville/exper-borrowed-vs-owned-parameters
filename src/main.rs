use exper_borrowed_vs_owned_parameters::{
    invoke_boxed_msgmf, invoke_boxed_msgmf_default, invoke_boxed_msgnf, invoke_boxed_msgnf_default,
    invoke_boxed_msgof, invoke_boxed_msgof_default, invoke_boxed_msgsf, invoke_boxed_msgsf_default,
    invoke_msgmf_borrowed, invoke_msgmf_default, invoke_msgmf_owned, invoke_msgnf_borrowed,
    invoke_msgnf_default, invoke_msgnf_owned, invoke_msgof_borrowed, invoke_msgof_default,
    invoke_msgof_owned, invoke_msgsf_borrowed, invoke_msgsf_default, invoke_msgsf_owned,
};

fn main() {
    invoke_msgnf_default();
    invoke_msgof_default();
    invoke_msgsf_default();
    invoke_msgmf_default();
    invoke_boxed_msgnf_default();
    invoke_boxed_msgof_default();
    invoke_boxed_msgsf_default();
    invoke_boxed_msgmf_default();
    invoke_msgnf_borrowed();
    invoke_msgnf_owned();
    invoke_msgof_borrowed();
    invoke_msgof_owned();
    invoke_msgsf_borrowed();
    invoke_msgsf_owned();
    invoke_msgmf_borrowed();
    invoke_msgmf_owned();
    invoke_boxed_msgnf();
    invoke_boxed_msgof();
    invoke_boxed_msgsf();
    invoke_boxed_msgmf();
}
