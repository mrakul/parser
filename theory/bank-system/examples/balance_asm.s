	.file	"balance.f2fef0de906b0a4f-cgu.0"
	.section	.text._ZN3std2rt10lang_start17h9cd4d2d19e3f3321E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E
	.globl	_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E
	.p2align	4
	.type	_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E,@function
_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	%ecx, %r8d
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.Lanon.d6a715fe02045263257e4a1efd7511e9.0(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17h74b643a2cc7fe3b4E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E, .Lfunc_end0-_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E","ax",@progbits
	.p2align	4
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E, .Lfunc_end1-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E
	.cfi_endproc

	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE,"ax",@progbits
	.p2align	4
	.type	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE,@function
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE, .Lfunc_end2-_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E","ax",@progbits
	.p2align	4
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E, .Lfunc_end3-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E
	.cfi_endproc

	.section	".text._ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE","ax",@progbits
	.p2align	4
	.type	_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE,@function
_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE:
	.cfi_startproc
	movq	(%rdi), %rdi
	jmpq	*_ZN4core3fmt17pointer_fmt_inner17h327f0f1fc2548254E@GOTPCREL(%rip)
.Lfunc_end4:
	.size	_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE, .Lfunc_end4-_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE
	.cfi_endproc

	.section	".text._ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE","ax",@progbits
	.p2align	4
	.type	_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE,@function
_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movq	(%rdi), %rbx
	movq	16(%rbx), %rsi
	testq	%rsi, %rsi
	je	.LBB5_2
	movq	24(%rbx), %rdi
	movl	$1, %edx
	callq	*_RNvCshXwFllX56pT_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB5_2:
	cmpq	$-1, %rbx
	je	.LBB5_4
	decq	8(%rbx)
	je	.LBB5_5
.LBB5_4:
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.LBB5_5:
	.cfi_def_cfa_offset 16
	movl	$40, %esi
	movl	$8, %edx
	movq	%rbx, %rdi
	popq	%rbx
	.cfi_def_cfa_offset 8
	jmpq	*_RNvCshXwFllX56pT_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.Lfunc_end5:
	.size	_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE, .Lfunc_end5-_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE
	.cfi_endproc

	.section	.text._ZN7balance4main17h1e1fd66fe5997d08E,"ax",@progbits
	.hidden	_ZN7balance4main17h1e1fd66fe5997d08E
	.globl	_ZN7balance4main17h1e1fd66fe5997d08E
	.p2align	4
	.type	_ZN7balance4main17h1e1fd66fe5997d08E,@function
_ZN7balance4main17h1e1fd66fe5997d08E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$64, %rsp
	.cfi_def_cfa_offset 112
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	callq	*_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$5, %edi
	movl	$1, %esi
	callq	*_RNvCshXwFllX56pT_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB6_17
	movb	$116, 4(%rax)
	movl	$1936877926, (%rax)
	movq	$5, 16(%rsp)
	movq	%rax, 24(%rsp)
	movq	$5, 32(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 40(%rsp)
	leaq	40(%rsp), %rbx
	movq	%rbx, 48(%rsp)
	leaq	_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE(%rip), %r12
	movq	%r12, 56(%rsp)
.Ltmp0:
	leaq	.Lanon.d6a715fe02045263257e4a1efd7511e9.2(%rip), %rdi
	leaq	48(%rsp), %rsi
	callq	*_ZN3std2io5stdio6_print17h526c462071e58c18E@GOTPCREL(%rip)
.Ltmp1:
	movq	16(%rsp), %r14
	movq	24(%rsp), %r15
	movq	32(%rsp), %r13
	callq	*_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$40, %edi
	movl	$8, %esi
	callq	*_RNvCshXwFllX56pT_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB6_3
	movq	$1, (%rax)
	movq	$1, 8(%rax)
	movq	%r14, 16(%rax)
	movq	%r15, 24(%rax)
	movq	%r13, 32(%rax)
	movq	%rax, 8(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 40(%rsp)
	movq	%rbx, 48(%rsp)
	movq	%r12, 56(%rsp)
.Ltmp3:
	leaq	.Lanon.d6a715fe02045263257e4a1efd7511e9.2(%rip), %rdi
	leaq	48(%rsp), %rsi
	callq	*_ZN3std2io5stdio6_print17h526c462071e58c18E@GOTPCREL(%rip)
.Ltmp4:
	movq	8(%rsp), %rax
	decq	(%rax)
	jne	.LBB6_12
	leaq	8(%rsp), %rdi
	callq	_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE
.LBB6_12:
	addq	$64, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB6_17:
	.cfi_def_cfa_offset 112
	movl	$1, %edi
	movl	$5, %esi
	callq	*_ZN5alloc7raw_vec12handle_error17hc03d75b8edc52e1bE@GOTPCREL(%rip)
.LBB6_3:
.Ltmp6:
	movl	$8, %edi
	movl	$40, %esi
	callq	*_ZN5alloc5alloc18handle_alloc_error17h841640659ea7a4e7E@GOTPCREL(%rip)
.Ltmp7:
	ud2
.LBB6_7:
.Ltmp5:
	movq	%rax, %rbx
	movq	8(%rsp), %rax
	decq	(%rax)
	jne	.LBB6_16
	leaq	8(%rsp), %rdi
	callq	_ZN5alloc2rc15Rc$LT$T$C$A$GT$9drop_slow17h49fa1a2064cd2c7eE
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
.LBB6_13:
.Ltmp2:
	movq	%rax, %rbx
	movq	16(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB6_16
	movq	24(%rsp), %rdi
	movl	$1, %edx
	jmp	.LBB6_15
.LBB6_5:
.Ltmp8:
	movq	%rax, %rbx
	testq	%r14, %r14
	je	.LBB6_16
	movl	$1, %edx
	movq	%r15, %rdi
	movq	%r14, %rsi
.LBB6_15:
	callq	*_RNvCshXwFllX56pT_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB6_16:
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end6:
	.size	_ZN7balance4main17h1e1fd66fe5997d08E, .Lfunc_end6-_ZN7balance4main17h1e1fd66fe5997d08E
	.cfi_endproc
	.section	.gcc_except_table._ZN7balance4main17h1e1fd66fe5997d08E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table6:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp4-.Lfunc_begin0
	.uleb128 .Ltmp6-.Ltmp4
	.byte	0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin0
	.uleb128 .Ltmp7-.Ltmp6
	.uleb128 .Ltmp8-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp7-.Lfunc_begin0
	.uleb128 .Lfunc_end6-.Ltmp7
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_ZN7balance4main17h1e1fd66fe5997d08E(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.Lanon.d6a715fe02045263257e4a1efd7511e9.0(%rip), %rsi
	movq	%rsp, %rdi
	xorl	%r8d, %r8d
	callq	*_ZN3std2rt19lang_start_internal17h74b643a2cc7fe3b4E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end7:
	.size	main, .Lfunc_end7-main
	.cfi_endproc

	.type	.Lanon.d6a715fe02045263257e4a1efd7511e9.0,@object
	.section	.data.rel.ro..Lanon.d6a715fe02045263257e4a1efd7511e9.0,"aw",@progbits
	.p2align	3, 0x0
.Lanon.d6a715fe02045263257e4a1efd7511e9.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E
	.size	.Lanon.d6a715fe02045263257e4a1efd7511e9.0, 48

	.type	.Lanon.d6a715fe02045263257e4a1efd7511e9.1,@object
	.section	.rodata..Lanon.d6a715fe02045263257e4a1efd7511e9.1,"a",@progbits
.Lanon.d6a715fe02045263257e4a1efd7511e9.1:
	.ascii	"first"
	.size	.Lanon.d6a715fe02045263257e4a1efd7511e9.1, 5

	.type	.Lanon.d6a715fe02045263257e4a1efd7511e9.2,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.d6a715fe02045263257e4a1efd7511e9.2:
	.asciz	"\bx addr: \300\001\n"
	.size	.Lanon.d6a715fe02045263257e4a1efd7511e9.2, 13

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.93.0 (254b59607 2026-01-19)"
	.section	".note.GNU-stack","",@progbits
