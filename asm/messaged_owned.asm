.section .text.exper_borrowed_vs_owned_parameters::message_owned,"ax",@progbits
	.p2align	4, 0x90
	.type	exper_borrowed_vs_owned_parameters::message_owned,@function
exper_borrowed_vs_owned_parameters::message_owned:

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 22
		fn message_owned(msg: Message) -> u32 {
	.cfi_startproc
	push rbx
	.cfi_def_cfa_offset 16
	.cfi_offset rbx, -16

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/vec/mod.rs : 2056
		self.len
	mov rax, qword ptr [rdi + 16]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/cmp.rs : 1464
		fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
	test rax, rax

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	je .LBB9_1

	mov rcx, qword ptr [rdi]
	cmp rax, 8
	jae .LBB9_5

	xor ebx, ebx
	xor edx, edx
	jmp .LBB9_4

.LBB9_1:
	xor ebx, ebx
	jmp .LBB9_13

.LBB9_5:
	mov rdx, rax
	and rdx, -8
	lea rsi, [rdx - 8]
	mov r8, rsi
	shr r8, 3
	inc r8
	test rsi, rsi
	je .LBB9_6

	mov rsi, r8
	and rsi, -2
	pxor xmm2, xmm2
	xor ebx, ebx
	pxor xmm0, xmm0
	pxor xmm1, xmm1

	.p2align	4, 0x90
.LBB9_8:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 28
		sum += msg.v[i] as u32
	movd xmm3, dword ptr [rcx + rbx]
	movd xmm4, dword ptr [rcx + rbx + 4]
	punpcklbw xmm3, xmm2
	punpcklwd xmm3, xmm2
	paddd xmm3, xmm0
	punpcklbw xmm4, xmm2
	punpcklwd xmm4, xmm2
	paddd xmm4, xmm1
	movd xmm0, dword ptr [rcx + rbx + 8]
	movd xmm1, dword ptr [rcx + rbx + 12]
	punpcklbw xmm0, xmm2
	punpcklwd xmm0, xmm2
	paddd xmm0, xmm3
	punpcklbw xmm1, xmm2
	punpcklwd xmm1, xmm2
	paddd xmm1, xmm4

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/num/uint_macros.rs : 470
		unsafe { intrinsics::unchecked_add(self, rhs) }
	add rbx, 16
	add rsi, -2
	jne .LBB9_8

	test r8b, 1
	je .LBB9_11

.LBB9_10:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 28
		sum += msg.v[i] as u32
	movd xmm2, dword ptr [rcx + rbx]
	movd xmm3, dword ptr [rcx + rbx + 4]
	pxor xmm4, xmm4
	punpcklbw xmm2, xmm4
	punpcklwd xmm2, xmm4
	paddd xmm0, xmm2
	punpcklbw xmm3, xmm4
	punpcklwd xmm3, xmm4
	paddd xmm1, xmm3

.LBB9_11:
		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	paddd xmm0, xmm1
	pshufd xmm1, xmm0, 238
	paddd xmm1, xmm0
	pshufd xmm0, xmm1, 85
	paddd xmm0, xmm1
	movd ebx, xmm0
	jmp .LBB9_12

.LBB9_6:
	pxor xmm0, xmm0
	xor ebx, ebx
	pxor xmm1, xmm1

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/num/uint_macros.rs : 470
		unsafe { intrinsics::unchecked_add(self, rhs) }
	test r8b, 1
	jne .LBB9_10
	jmp .LBB9_11

.LBB9_4:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 28
		sum += msg.v[i] as u32
	movzx esi, byte ptr [rcx + rdx]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/num/uint_macros.rs : 470
		unsafe { intrinsics::unchecked_add(self, rhs) }
	inc rdx

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 28
		sum += msg.v[i] as u32
	add ebx, esi

.LBB9_12:
	cmp rax, rdx
		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	jne .LBB9_4

.LBB9_13:
		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/ptr/mod.rs : 490
		pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
	mov rsi, qword ptr [rdi + 8]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/raw_vec.rs : 241
		if T::IS_ZST || self.cap == 0 {
	test rsi, rsi
	je .LBB9_15

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/ptr/mod.rs : 490
		pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
	mov rdi, qword ptr [rdi]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/alloc/layout.rs : 452
		if element_size != 0 && n > Layout::max_size_for_align(align) / element_size {
	mov rdx, rsi
	not rdx
	shr rdx, 63

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/alloc/src/alloc.rs : 113
		unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
	call qword ptr [rip + __rust_dealloc@GOTPCREL]

.LBB9_15:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 31
		}
	mov eax, ebx
	pop rbx
	.cfi_def_cfa_offset 8
	ret

	.size	exper_borrowed_vs_owned_parameters::message_owned, .Lfunc_end9-exper_borrowed_vs_owned_parameters::message_owned
