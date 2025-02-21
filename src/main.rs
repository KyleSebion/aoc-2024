use std::time::Instant;

use cudarc::{
    driver::{CudaDevice, DeviceSlice, DriverError, LaunchAsync, LaunchConfig},
    nvrtc::compile_ptx,
};

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
    "
}
fn convert_to_res(v: u64) -> String {
    let mut v = v;
    let mut s = String::new();
    loop {
        if v == 15 {
            break;
        }
        if !s.is_empty() { s.push(','); }
        let c = (v & 0b111 | 0x30) as u8 as char;
        s.push(c);
        v >>= 3;
    }
    s.chars().rev().collect()
}
fn main() -> Result<(), DriverError> {
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
                println!("{} {}; ", i, convert_to_res(*v));
            }
        }
    }
    println!("END {:?}", s.elapsed());
    Ok(())
}
