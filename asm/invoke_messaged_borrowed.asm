.section .text.exper_borrowed_vs_owned_parameters::invoke_message_borrowed,"ax",@progbits
	.globl	exper_borrowed_vs_owned_parameters::invoke_message_borrowed
	.p2align	4, 0x90
	.type	exper_borrowed_vs_owned_parameters::invoke_message_borrowed,@function
exper_borrowed_vs_owned_parameters::invoke_message_borrowed:

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 40
		pub fn invoke_message_borrowed() {
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	push rbp
	.cfi_def_cfa_offset 16
	push r14
	.cfi_def_cfa_offset 24
	push rbx
	.cfi_def_cfa_offset 32
	sub rsp, 80
	.cfi_def_cfa_offset 112
	.cfi_offset rbx, -32
	.cfi_offset r14, -24
	.cfi_offset rbp, -16
	lea rdi, [rsp + 32]

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 41
		let msg = Message { v: random_array(5, 0, 20) };
	xor esi, esi
	mov edx, 20
	call exper_borrowed_vs_owned_parameters::random_array

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 42
		let r1 = message_borrowed(&msg);
	mov r14, qword ptr [rsp + 32]
	mov rsi, qword ptr [rsp + 48]

	mov rdi, r14
	call exper_borrowed_vs_owned_parameters::message_borrowed

	mov ebp, eax

	lea rdi, [rsp + 56]

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 43
		let msg = Message { v: random_array(5, 30, 50) };
	mov esi, 30
	mov edx, 50
	call exper_borrowed_vs_owned_parameters::random_array

	mov rsi, qword ptr [rsp + 72]
	mov qword ptr [rsp + 16], rsi
	movups xmm0, xmmword ptr [rsp + 56]
	movaps xmmword ptr [rsp], xmm0

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 44
		let r2 = message_borrowed(&msg);
	mov rbx, qword ptr [rsp]

	mov rdi, rbx
	call exper_borrowed_vs_owned_parameters::message_borrowed

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 45
		assert!(r1 != r2);
	cmp ebp, eax
	je .LBB11_11

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/ptr/mod.rs : 490
		pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
	mov rsi, qword ptr [rsp + 8]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/raw_vec.rs : 241
		if T::IS_ZST || self.cap == 0 {
	test rsi, rsi
	je .LBB11_6

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/alloc/layout.rs : 452
		if element_size != 0 && n > Layout::max_size_for_align(align) / element_size {
	mov rdx, rsi
	not rdx
	shr rdx, 63

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/alloc.rs : 113
		unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
	mov rdi, rbx
	call qword ptr [rip + __rust_dealloc@GOTPCREL]

.LBB11_6:
		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/ptr/mod.rs : 490
		pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
	mov rsi, qword ptr [rsp + 40]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/raw_vec.rs : 241
		if T::IS_ZST || self.cap == 0 {
	test rsi, rsi
	je .LBB11_13

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/alloc/layout.rs : 452
		if element_size != 0 && n > Layout::max_size_for_align(align) / element_size {
	mov rdx, rsi
	not rdx
	shr rdx, 63

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/alloc.rs : 113
		unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
	mov rdi, r14
	add rsp, 80
	.cfi_def_cfa_offset 32
	pop rbx
	.cfi_def_cfa_offset 24
	pop r14

	.cfi_def_cfa_offset 16
	pop rbp
	.cfi_def_cfa_offset 8
	jmp qword ptr [rip + __rust_dealloc@GOTPCREL]

.LBB11_13:
	.cfi_def_cfa_offset 112
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 46
		}
	add rsp, 80
	.cfi_def_cfa_offset 32
	pop rbx
	.cfi_def_cfa_offset 24
	pop r14
	.cfi_def_cfa_offset 16
	pop rbp
	.cfi_def_cfa_offset 8
	ret
.LBB11_11:
	.cfi_def_cfa_offset 112

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 45
		assert!(r1 != r2);
	lea rdi, [rip + .L__unnamed_5]
	lea rdx, [rip + .L__unnamed_6]
	mov esi, 26
	call qword ptr [rip + core::panicking::panic@GOTPCREL]

	ud2

	mov rbx, rax
	mov rdi, rsp
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 46
		}
	call core::ptr::drop_in_place<exper_borrowed_vs_owned_parameters::Message>
	jmp .LBB11_9

	mov rbx, rax
.LBB11_9:
	lea rdi, [rsp + 32]
	call core::ptr::drop_in_place<exper_borrowed_vs_owned_parameters::Message>
	mov rdi, rbx
	call _Unwind_Resume@PLT
	ud2

	.size	exper_borrowed_vs_owned_parameters::invoke_message_borrowed, .Lfunc_end11-exper_borrowed_vs_owned_parameters::invoke_message_borrowed
