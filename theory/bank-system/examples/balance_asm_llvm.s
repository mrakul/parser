; ModuleID = 'balance.f2fef0de906b0a4f-cgu.0'
source_filename = "balance.f2fef0de906b0a4f-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E" }>, align 8
@alloc_4986dd618090c125bc3e853ec5468cc0 = private unnamed_addr constant [5 x i8] c"first", align 1
@alloc_fe46d3b457ff7fae0be600dbe96da163 = private unnamed_addr constant [13 x i8] c"\08x addr: \C0\01\0A\00", align 1
@alloc_7e350088082a319207da5ed595ffdebb = private unnamed_addr constant [6 x i8] c"Second", align 1

; std::rt::lang_start
; Function Attrs: nonlazybind uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17h9cd4d2d19e3f3321E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_ZN3std2rt19lang_start_internal17h74b643a2cc7fe3b4E(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE(ptr noundef nonnull %_4) #10
  ret i32 0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline nonlazybind uwtable
define internal fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #11, !srcloc !5
  ret void
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint nonlazybind uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h495a08bdbdc8dc90E"(ptr noundef readonly captures(none) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace
  tail call fastcc void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h4a0e861d337904cfE(ptr noundef nonnull readonly %0) #10, !noalias !6
  ret i32 0
}

; <*const T as core::fmt::Pointer>::fmt
; Function Attrs: nonlazybind uwtable
define internal noundef zeroext i1 @"_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE"(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %self1 = load ptr, ptr %self, align 8, !noundef !4
  %_4 = ptrtoint ptr %self1 to i64
; call core::fmt::pointer_fmt_inner
  %0 = tail call noundef zeroext i1 @_ZN4core3fmt17pointer_fmt_inner17h327f0f1fc2548254E(i64 noundef %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %0
}

; balance::main
; Function Attrs: nonlazybind uwtable
define hidden void @_ZN7balance4main17h1e1fd66fe5997d08E() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %args2 = alloca [16 x i8], align 8
  %_13 = alloca [8 x i8], align 8
  %x1 = alloca [24 x i8], align 8
  %args = alloca [16 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  %x = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %x)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #11, !noalias !9
; call __rustc::__rust_alloc
  %0 = tail call noundef dereferenceable_or_null(5) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 5, 7) 5, i64 noundef range(i64 1, -9223372036854775807) 1) #11, !noalias !9
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb3.i.i, label %"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E.exit", !prof !15

bb3.i.i:                                          ; preds = %start
; call alloc::raw_vec::handle_error
  tail call void @_ZN5alloc7raw_vec12handle_error17hc03d75b8edc52e1bE(i64 noundef 1, i64 range(i64 5, 7) 5) #12, !noalias !16
  unreachable

"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E.exit": ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(5) %0, ptr noundef nonnull align 1 dereferenceable(5) @alloc_4986dd618090c125bc3e853ec5468cc0, i64 5, i1 false), !noalias !17
  store i64 5, ptr %x, align 8
  %_17.sroa.4.0.x.sroa_idx = getelementptr inbounds nuw i8, ptr %x, i64 8
  store ptr %0, ptr %_17.sroa.4.0.x.sroa_idx, align 8
  %_17.sroa.5.0.x.sroa_idx = getelementptr inbounds nuw i8, ptr %x, i64 16
  store i64 5, ptr %_17.sroa.5.0.x.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5)
  store ptr %x, ptr %_5, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_5, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @"_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE", ptr %_7.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h526c462071e58c18E(ptr noundef nonnull @alloc_fe46d3b457ff7fae0be600dbe96da163, ptr noundef nonnull %args)
          to label %bb1 unwind label %cleanup

bb6:                                              ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i14", %cleanup3, %cleanup
  %.pn = phi { ptr, i32 } [ %2, %cleanup ], [ %5, %cleanup3 ], [ %5, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i14" ]
  %x.val = load i64, ptr %x, align 8
  %_6.i.i.i.i4.i.i = icmp eq i64 %x.val, 0
  br i1 %_6.i.i.i.i4.i.i, label %bb7, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i": ; preds = %bb6
  %x.val5 = load ptr, ptr %_17.sroa.4.0.x.sroa_idx, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %x.val5, i64 noundef %x.val, i64 noundef range(i64 1, -9223372036854775807) 1) #11
  br label %bb7

cleanup:                                          ; preds = %bb3.i.i11, %"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E.exit"
  %2 = landingpad { ptr, i32 }
          cleanup
  br label %bb6

bb1:                                              ; preds = %"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E.exit"
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %x1)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #11, !noalias !18
; call __rustc::__rust_alloc
  %3 = call noundef dereferenceable_or_null(6) ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef range(i64 5, 7) 6, i64 noundef range(i64 1, -9223372036854775807) 1) #11, !noalias !18
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb3.i.i11, label %bb9, !prof !15

bb3.i.i11:                                        ; preds = %bb1
; invoke alloc::raw_vec::handle_error
  invoke void @_ZN5alloc7raw_vec12handle_error17hc03d75b8edc52e1bE(i64 noundef 1, i64 range(i64 5, 7) 6) #12
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb3.i.i11
  unreachable

bb9:                                              ; preds = %bb1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(6) %3, ptr noundef nonnull align 1 dereferenceable(6) @alloc_7e350088082a319207da5ed595ffdebb, i64 6, i1 false), !noalias !24
  store i64 6, ptr %x1, align 8
  %_27.sroa.4.0.x1.sroa_idx = getelementptr inbounds nuw i8, ptr %x1, i64 8
  store ptr %3, ptr %_27.sroa.4.0.x1.sroa_idx, align 8
  %_27.sroa.5.0.x1.sroa_idx = getelementptr inbounds nuw i8, ptr %x1, i64 16
  store i64 6, ptr %_27.sroa.5.0.x1.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_13)
  store ptr %x1, ptr %_13, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %_13, ptr %args2, align 8
  %_15.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args2, i64 8
  store ptr @"_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h7e135808e04dfa2eE", ptr %_15.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h526c462071e58c18E(ptr noundef nonnull @alloc_fe46d3b457ff7fae0be600dbe96da163, ptr noundef nonnull %args2)
          to label %bb2 unwind label %cleanup3

cleanup3:                                         ; preds = %bb9
  %5 = landingpad { ptr, i32 }
          cleanup
  %x1.val = load i64, ptr %x1, align 8
  %_6.i.i.i.i4.i.i13 = icmp eq i64 %x1.val, 0
  br i1 %_6.i.i.i.i4.i.i13, label %bb6, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i14"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i14": ; preds = %cleanup3
  %x1.val6 = load ptr, ptr %_27.sroa.4.0.x1.sroa_idx, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %x1.val6, i64 noundef %x1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #11
  br label %bb6

bb2:                                              ; preds = %bb9
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_13)
  %x1.val9 = load i64, ptr %x1, align 8
  %_6.i.i.i.i4.i.i16 = icmp eq i64 %x1.val9, 0
  br i1 %_6.i.i.i.i4.i.i16, label %bb3, label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i17"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i17": ; preds = %bb2
  %x1.val10 = load ptr, ptr %_27.sroa.4.0.x1.sroa_idx, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %x1.val10, i64 noundef %x1.val9, i64 noundef range(i64 1, -9223372036854775807) 1) #11
  br label %bb3

bb3:                                              ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i17", %bb2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %x1)
  %x.val7 = load i64, ptr %x, align 8
  %_6.i.i.i.i4.i.i19 = icmp eq i64 %x.val7, 0
  br i1 %_6.i.i.i.i4.i.i19, label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4758211e487babf3E.exit21", label %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i20"

"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i20": ; preds = %bb3
  %x.val8 = load ptr, ptr %_17.sroa.4.0.x.sroa_idx, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr noundef nonnull %x.val8, i64 noundef %x.val7, i64 noundef range(i64 1, -9223372036854775807) 1) #11
  br label %"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4758211e487babf3E.exit21"

"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4758211e487babf3E.exit21": ; preds = %bb3, %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i20"
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %x)
  ret void

bb7:                                              ; preds = %"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h5320a00831908767E.exit.i.i.i5.i.i", %bb6
  resume { ptr, i32 } %.pn
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #3

; std::rt::lang_start_internal
; Function Attrs: nonlazybind uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17h74b643a2cc7fe3b4E(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #3

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #4

; Function Attrs: nounwind nonlazybind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #5

; core::fmt::pointer_fmt_inner
; Function Attrs: nonlazybind uwtable
declare noundef zeroext i1 @_ZN4core3fmt17pointer_fmt_inner17h327f0f1fc2548254E(i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind nonlazybind uwtable
declare void @_RNvCshXwFllX56pT_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #5

; __rustc::__rust_alloc
; Function Attrs: nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCshXwFllX56pT_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #6

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn nonlazybind optsize uwtable
declare void @_ZN5alloc7raw_vec12handle_error17hc03d75b8edc52e1bE(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #7

; __rustc::__rust_dealloc
; Function Attrs: nounwind nonlazybind allockind("free") uwtable
declare void @_RNvCshXwFllX56pT_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #8

; std::io::stdio::_print
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std2io5stdio6_print17h526c462071e58c18E(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; Function Attrs: nonlazybind
define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #9 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_ZN7balance4main17h1e1fd66fe5997d08E, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_ZN3std2rt19lang_start_internal17h74b643a2cc7fe3b4E(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { noinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { nounwind nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #6 = { nounwind nonlazybind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCshXwFllX56pT_7___rustc19___rust_alloc_zeroed" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #7 = { cold minsize noreturn nonlazybind optsize uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #8 = { nounwind nonlazybind allockind("free") uwtable "alloc-family"="__rust_alloc" "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #9 = { nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #10 = { noinline }
attributes #11 = { nounwind }
attributes #12 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
!3 = !{!"rustc version 1.93.0 (254b59607 2026-01-19)"}
!4 = !{}
!5 = !{i64 13246748590804918}
!6 = !{!7}
!7 = distinct !{!7, !8, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E: %_1"}
!8 = distinct !{!8, !"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc83513193c273bf3E"}
!9 = !{!10, !12, !14}
!10 = distinct !{!10, !11, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hbd57a3e9acb0a288E: %_0"}
!11 = distinct !{!11, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hbd57a3e9acb0a288E"}
!12 = distinct !{!12, !13, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E: %v"}
!13 = distinct !{!13, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E"}
!14 = distinct !{!14, !13, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E: %s.0"}
!15 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!16 = !{!12, !14}
!17 = !{!12}
!18 = !{!19, !21, !23}
!19 = distinct !{!19, !20, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hbd57a3e9acb0a288E: %_0"}
!20 = distinct !{!20, !"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17hbd57a3e9acb0a288E"}
!21 = distinct !{!21, !22, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E: %v"}
!22 = distinct !{!22, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E"}
!23 = distinct !{!23, !22, !"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hb00013f05419e4d1E: %s.0"}
!24 = !{!21}
