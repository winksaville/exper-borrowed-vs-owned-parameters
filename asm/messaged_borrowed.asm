.section .text.exper_borrowed_vs_owned_parameters::message_borrowed,"ax",@progbits
	.p2align	4, 0x90
	.type	exper_borrowed_vs_owned_parameters::message_borrowed,@function
exper_borrowed_vs_owned_parameters::message_borrowed:

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 10
		fn message_borrowed(msg: &Message) -> u32 {
	.cfi_startproc
		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/cmp.rs : 1464
		fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
	test rsi, rsi

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	je .LBB8_1

	cmp rsi, 8
	jae .LBB8_4

	xor eax, eax
	xor ecx, ecx
	jmp .LBB8_11
.LBB8_1:
	xor eax, eax
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 19
		}
	ret
.LBB8_4:

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	mov rcx, rsi
	and rcx, -8
	lea rax, [rcx - 8]
	mov r8, rax
	shr r8, 3
	inc r8
	test rax, rax
	je .LBB8_5

	mov rax, r8
	and rax, -2
	pxor xmm2, xmm2
	xor edx, edx
	pxor xmm0, xmm0
	pxor xmm1, xmm1

	.p2align	4, 0x90
.LBB8_7:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 16
		sum += msg.v[i] as u32
	movd xmm3, dword ptr [rdi + rdx]
	movd xmm4, dword ptr [rdi + rdx + 4]
	punpcklbw xmm3, xmm2
	punpcklwd xmm3, xmm2
	paddd xmm3, xmm0
	punpcklbw xmm4, xmm2
	punpcklwd xmm4, xmm2
	paddd xmm4, xmm1
	movd xmm0, dword ptr [rdi + rdx + 8]
	movd xmm1, dword ptr [rdi + rdx + 12]
	punpcklbw xmm0, xmm2
	punpcklwd xmm0, xmm2
	paddd xmm0, xmm3
	punpcklbw xmm1, xmm2
	punpcklwd xmm1, xmm2
	paddd xmm1, xmm4

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/num/uint_macros.rs : 470
		unsafe { intrinsics::unchecked_add(self, rhs) }
	add rdx, 16
	add rax, -2
	jne .LBB8_7

	test r8b, 1
	je .LBB8_10

.LBB8_9:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 16
		sum += msg.v[i] as u32
	movd xmm2, dword ptr [rdi + rdx]
	movd xmm3, dword ptr [rdi + rdx + 4]
	pxor xmm4, xmm4
	punpcklbw xmm2, xmm4
	punpcklwd xmm2, xmm4
	paddd xmm0, xmm2
	punpcklbw xmm3, xmm4
	punpcklwd xmm3, xmm4
	paddd xmm1, xmm3

.LBB8_10:
		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	paddd xmm0, xmm1
	pshufd xmm1, xmm0, 238
	paddd xmm1, xmm0
	pshufd xmm0, xmm1, 85
	paddd xmm0, xmm1
	movd eax, xmm0
	cmp rcx, rsi
	je .LBB8_12

	.p2align	4, 0x90
.LBB8_11:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 16
		sum += msg.v[i] as u32
	movzx edx, byte ptr [rdi + rcx]

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/num/uint_macros.rs : 470
		unsafe { intrinsics::unchecked_add(self, rhs) }
	inc rcx

		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 16
		sum += msg.v[i] as u32
	add eax, edx

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/cmp.rs : 1464
		fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
	cmp rsi, rcx

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/iter/range.rs : 621
		if self.start < self.end {
	jne .LBB8_11

.LBB8_12:
		// /home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters/src/lib.rs : 19
		}
	ret
.LBB8_5:

	pxor xmm0, xmm0
	xor edx, edx
	pxor xmm1, xmm1

		// /rustc/ff8c8dfbe66701531e3e5e335c28c544d0fbc945/library/core/src/num/uint_macros.rs : 470
		unsafe { intrinsics::unchecked_add(self, rhs) }
	test r8b, 1
	jne .LBB8_9
	jmp .LBB8_10

	.size	exper_borrowed_vs_owned_parameters::message_borrowed, .Lfunc_end8-exper_borrowed_vs_owned_parameters::message_borrowed
