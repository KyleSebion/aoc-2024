#![allow(dead_code)]
use std::time::Instant;
use cudarc::{
    driver::{CudaDevice, DeviceSlice, DriverError, LaunchAsync, LaunchConfig},
    nvrtc::compile_ptx,
};
use chrono::prelude::*;


fn prog_ptx() -> &'static str {
    "
extern \"C\" __device__ unsigned long long int * combo(unsigned long long int *op, unsigned long long int *a, unsigned long long int *b, unsigned long long int *c) {
    switch (*op) {
        case 4:
            return a;
        case 5:
            return b;
        case 6:
            return c;
    }
    return op;
}
extern \"C\" __device__ unsigned long long int pow2(unsigned long long int exp) {
    return 1 << exp;
}
extern \"C\" __global__ void prog(unsigned long long int base, unsigned long long int *out, int cnt) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int ip = 0;
    unsigned long long int a  = i + base;
    unsigned long long int b  = 0;
    unsigned long long int c  = 0;
    unsigned long long int out_val = 15;
    int instrs[] = { 2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 2, 5, 5, 3, 0 };
    int instr_edge = 16 - 1;

    if (i < cnt) {
        while (ip < instr_edge) {
            int instr = instrs[ip];
            unsigned long long int op = instrs[ip + 1];
            switch (instr) {
                case 0:
                    a /= pow2(*combo(&op, &a, &b, &c));
                    ip += 2;
                    break;
                case 1:
                    b ^= op;
                    ip += 2;
                    break;
                case 2:
                    b = *combo(&op, &a, &b, &c) % 8;
                    ip += 2;
                    break;
                case 3:
                    if (a == 0) {
                        ip += 2;
                    } else {
                        ip = op;
                    }
                    break;
                case 4:
                    b ^= c;
                    ip += 2;
                    break;
                case 5:
                    out_val = (out_val << 3) | (*combo(&op, &a, &b, &c) % 8);
                    ip += 2;
                    break;
                case 6:
                    b = a / pow2(*combo(&op, &a, &b, &c));
                    ip += 2;
                    break;
                case 7:
                    c = a / pow2(*combo(&op, &a, &b, &c));
                    ip += 2;
                    break;
            }
        }
        out[i] = out_val;
    }
}
extern \"C\" __global__ void prog2(unsigned long long int base, unsigned long long int *out, int cnt) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned long long int ib = i + base;
    unsigned int ip = 0;
    unsigned long long int a  = i + base;
    unsigned long long int b  = 0;
    unsigned long long int c  = 0;
    unsigned long long int out_val = 15;
    int instrs[] = { 2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 2, 5, 5, 3, 0 };
    int instr_edge = 16 - 1;

    if (i < cnt) {
        while (ip < instr_edge) {
            int instr = instrs[ip];
            unsigned long long int op = instrs[ip + 1];
            switch (instr) {
                case 0:
                    a = a >> *combo(&op, &a, &b, &c);
                    ip += 2;
                    break;
                case 1:
                    b ^= op;
                    ip += 2;
                    break;
                case 2:
                    b = *combo(&op, &a, &b, &c) % 8;
                    ip += 2;
                    break;
                case 3:
                    if (a == 0) {
                        ip += 2;
                    } else {
                        ip = op;
                    }
                    break;
                case 4:
                    b ^= c;
                    ip += 2;
                    break;
                case 5:
                    out_val = (out_val << 3) | (*combo(&op, &a, &b, &c) % 8);
                    ip += 2;
                    break;
                case 6:
                    b = a >> *combo(&op, &a, &b, &c);
                    ip += 2;
                    break;
                case 7:
                    c = a >> *combo(&op, &a, &b, &c);
                    ip += 2;
                    break;
            }
        }
        if (out_val == 4311044668140376) {
            while (1) {
                unsigned long long int old = atomicAdd(out, 0);
                if (old != 0 && old < ib) { break; }
                unsigned long long int maybeNew = atomicCAS(out, old, ib);
                if (maybeNew == ib) { break; }
            }
        }
    }
}
    "
}
/*
641428842 3,3,3,3,3,3,4,1,1,5   16566302797
926641514 3,3,3,3,3,3,4,1,1,5   16566302797
931135488 3,3,3,3,3,3,4,1,1,5   16566302797
1020263277 3,3,3,3,3,3,4,1,1,5  16566302797
3,3,3,3,3,3,4,1,1,5==3,3,3,3,3,3,4,1,1,5 = true, 16566302797==16566302797 = true
2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0==2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0 = true, 4311044668140376==4311044668140376 = true
931135488 3,3,3,3,3,3,4,1,1,5
[UInt64]$v = 15; $a=3,3,3,3,3,3,4,1,1,5; $a | %{ $v = ($v -shl 3) -bor $_ }; $v
  16566302797
$r=''; $v = 16566302797; while ($v -ne 15) { if ($r -ne '') { $r = ',' + $r }; $r = '{0}{1}' -f ($v -band 7),$r; $v = $v -shr 3; }; $r
  3,3,3,3,3,3,4,1,1,5
 */
fn num_res_to_str(v: u64) -> Option<String> {
    if (v << v.leading_zeros()).leading_ones() < 4 { return None; }
    let mut v = v;
    let mut s = String::new();
    loop {
        if v == 15 { break; }
        if !s.is_empty() { s.push(','); }
        let c = (v & 0b111 | 0x30) as u8 as char;
        s.push(c);
        v >>= 3;
    }
    Some(s.chars().rev().collect())
}
fn str_res_to_num(res: &str) -> u64 {
    let mut v = 15;
    for r in res.split(",").map(|r| r.parse::<u64>().unwrap()) {
        v = v << 3 | r;
    }
    v
}
fn cuda_all_results_vec() -> Result<(), DriverError> {
    // cuda: ~224,757,102 per sec (7x faster)
    // mt:    ~32,343,872 per sec
    println!("START");
    const CUDA_SIZE: u64 = 1024*1024*222; // max in release until gpu OOM error
    const LOOP_END: u64 = 16;
    let s = Instant::now();
    let dev = CudaDevice::new(0)?;
    dev.load_ptx(compile_ptx(prog_ptx()).unwrap(), "prog", &["prog"])?;
    let f = dev.get_func("prog", "prog").unwrap();
    let a_dev = dev.htod_copy([0_u64; CUDA_SIZE as usize].into())?;
    let n = a_dev.len() as u32;
    let cfg = LaunchConfig::for_num_elems(n);
    for s in (0_u64..LOOP_END).map(|s| s * CUDA_SIZE) {
        let mut a_dev = a_dev.clone();
        unsafe { f.clone().launch(cfg, (s, &mut a_dev, n)) }?;
        let a_host_2 = dev.sync_reclaim(a_dev)?;
        for (i, v) in a_host_2.iter().enumerate() {
            let i = i as u64 + s;
            if i % CUDA_SIZE == 0 || i == CUDA_SIZE * LOOP_END - 1 {
                if let Some(r) = num_res_to_str(*v) {
                    println!("{} {}; ", i, r);
                } else {
                    println!("{} N/A; ", i);
                }
            }
        }
    }
    println!("END {:?}", s.elapsed());
    Ok(())
}
fn cuda_only_good(loop_start: u64, loop_end: u64) -> Result<u64, DriverError> {
    println!("START");
    const CUDA_SIZE: u64 = 1024*1024*222; // max in release until gpu OOM error

    // const LOOP_END: u64 = LOOP_START + 100; // 70s

    //lowest good: 470646    109558863325594
    // const LOOP_START: u64 = 109558863325594 / CUDA_SIZE;
    // const LOOP_END: u64 = 109558863325594_u64.div_ceil(CUDA_SIZE);

    //took awhile in normal rust (probably over an hour) and found none: 108995544197530..=109146941794714
    //  should be ~7.5 minutes in cuda (actual: 451.9383835s ~7.5 minutes)
    //  335,316,287 per second
    //  388,803,375 per second after i switched "/pow2(v)" to ">>v"
    //  none found
    // const LOOP_START: u64 = 108995544197530 / CUDA_SIZE;
    // const LOOP_END: u64 = 109146941794714_u64.div_ceil(CUDA_SIZE);

    let mut res = 0;
    let s = Instant::now();
    let dev = CudaDevice::new(0)?;
    dev.load_ptx(compile_ptx(prog_ptx()).unwrap(), "prog", &["prog2"])?;
    let f = dev.get_func("prog", "prog2").unwrap();
    let a_dev = dev.htod_copy([0_u64; 1].into())?;
    let n = CUDA_SIZE as u32;
    let cfg = LaunchConfig::for_num_elems(n);
    for s in (loop_start..loop_end).map(|s| s * CUDA_SIZE) {
        let mut a_dev = a_dev.clone();
        unsafe { f.clone().launch(cfg, (s, &mut a_dev, n)) }?;
        let a_host_2 = dev.sync_reclaim(a_dev)?;
        if a_host_2[0] != 0 {
            println!("{}", a_host_2[0]);
            res = a_host_2[0];
            break;
        }
    }
    println!("END {:?}", s.elapsed());
    Ok(res)
}
fn main() {
    // cuda_all_results_vec()
    // let mut loop_start = 470646; // down to 469696..469746: none
    // let mut loop_start = 469696; // down to 468746..468796: none
    // let mut loop_start = 468746; // down to 465146..465196: 108285405522330
    // let mut loop_start = 465146; // down to 455696..455746: 106086382266778
    // 106086382266778 is the answer!
    let mut loop_start = 455696;
    let loop_cnt = 50;
    let s = Instant::now();
    loop {
        let loop_end = loop_start + loop_cnt;
        println!("{} {loop_start} {loop_end} {:?}", Local::now(), s.elapsed());
        if let Ok(v) = cuda_only_good(loop_start, loop_end) {
            if v != 0 && v != 109558863325594 && v != 108285405522330 {
                println!("Found 1 {v} {loop_start}..{loop_end}");
                break;
            }
        }
        if loop_start == 0 {
            break;
        }
        loop_start = loop_start.saturating_sub(loop_cnt);
    }
}

/*

*/
