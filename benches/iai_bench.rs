use exper_borrowed_vs_owned_parameters::{
    invoke_msgmf_borrowed, invoke_msgmf_default, invoke_msgmf_owned, invoke_msgnf_borrowed,
    invoke_msgnf_default, invoke_msgnf_owned, invoke_msgof_borrowed, invoke_msgof_default,
    invoke_msgof_owned, invoke_msgsf_borrowed, invoke_msgsf_default, invoke_msgsf_owned,
};

iai::main!(
    invoke_msgnf_default,
    invoke_msgof_default,
    invoke_msgsf_default,
    invoke_msgmf_default,
    invoke_msgnf_borrowed,
    invoke_msgnf_owned,
    invoke_msgof_borrowed,
    invoke_msgof_owned,
    invoke_msgsf_borrowed,
    invoke_msgsf_owned,
    invoke_msgmf_borrowed,
    invoke_msgmf_owned,
);
