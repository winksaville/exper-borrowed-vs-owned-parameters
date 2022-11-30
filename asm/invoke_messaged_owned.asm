.section .text.exper_borrowed_vs_owned_parameters::invoke_message_owned,"ax",@progbits
	.globl	exper_borrowed_vs_owned_parameters::invoke_message_owned
	.p2align	4, 0x90
	.type	exper_borrowed_vs_owned_parameters::invoke_message_owned,@function
exper_borrowed_vs_owned_parameters::invoke_message_owned:

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 49
		pub fn invoke_message_owned() {
	.cfi_startproc
	push rbx
	.cfi_def_cfa_offset 16
	sub rsp, 80
	.cfi_def_cfa_offset 96
	.cfi_offset rbx, -16
	lea rdi, [rsp + 32]

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 50
		let msg = Message { v: random_array(5, 0, 20) };
	xor esi, esi
	mov edx, 20
	call exper_borrowed_vs_owned_parameters::random_array

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 51
		let r1 = message_owned(msg);
	mov rax, qword ptr [rsp + 48]
	mov qword ptr [rsp + 16], rax
	movups xmm0, xmmword ptr [rsp + 32]
	movaps xmmword ptr [rsp], xmm0
	mov rdi, rsp
	call exper_borrowed_vs_owned_parameters::message_owned
	mov ebx, eax

	lea rdi, [rsp + 56]

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 52
		let msg = Message { v: random_array(5, 30, 50) };
	mov esi, 30
	mov edx, 50
	call exper_borrowed_vs_owned_parameters::random_array

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 53
		let r2 = message_owned(msg);
	mov rax, qword ptr [rsp + 72]
	mov qword ptr [rsp + 16], rax
	movups xmm0, xmmword ptr [rsp + 56]
	movaps xmmword ptr [rsp], xmm0
	mov rdi, rsp
	call exper_borrowed_vs_owned_parameters::message_owned

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 54
		assert!(r1 != r2);
	cmp ebx, eax
	je .LBB12_2

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 55
		}
	add rsp, 80
	.cfi_def_cfa_offset 16
	pop rbx
	.cfi_def_cfa_offset 8
	ret
.LBB12_2:
	.cfi_def_cfa_offset 96

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 54
		assert!(r1 != r2);
	lea rdi, [rip + .L__unnamed_5]
	lea rdx, [rip + .L__unnamed_7]
	mov esi, 26
	call qword ptr [rip + core::panicking::panic@GOTPCREL]

	ud2

	.size	exper_borrowed_vs_owned_parameters::invoke_message_owned, .Lfunc_end12-exper_borrowed_vs_owned_parameters::invoke_message_owned
