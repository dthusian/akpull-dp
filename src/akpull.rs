use std::mem::swap;
use crate::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum AccelType {
  None,
  AVX2
}

pub struct AkPullArgs {
  pub pulls: u64,
  pub max_count: u32,
  pub base_rate: f64
}

pub struct AkPullResult {
  pub n6: Vec<u64>
}

fn pity(bruh: usize) -> f64 {
  if bruh < 50 {
    0.02
  } else {
    (bruh - 48) as f64 * 0.02
  }
}

pub fn dp_noaccel(args: AkPullArgs) -> AkPullResult {
  let state_size = 100 * args.max_count as usize;
  // [n6 * 100 + pity]
  let mut state0 = vec![0u64; state_size];
  let mut state1 = vec![0u64; state_size];
  let mut overmax = 0u64;
  state0[0] = u32::MAX as u64;
  for i in 0..args.pulls as usize {
    println!("{:?}", state0);
    for j in 0..args.max_count as usize {
      // zero pity draws 0.02 from everyone else
      if j > 0 {
        state1[j * 100] = 0;
        for k in 0..99 {
          state1[j * 100] += f64::round(state0[(j - 1) * 100 + k] as f64 * pity(k)) as u64;
        }
      } else {
        state1[j * 100] = 0;
      }
      for k in 0..100usize {
        overmax += f64::round(state0[(args.max_count as usize - 1) * 100 + k] as f64 * pity(k)) as u64;
      }

      // >0 pity draws 0.98 from prev
      for k in 1..100 {
        state1[j * 100 + k] = f64::round(state0[j * 100 + k - 1] as f64 * (1. - pity(k - 1))) as u64;
      }
    }
    swap(&mut state0, &mut state1);
  }
  let mut n6 = vec![0u64; (args.max_count + 1) as usize];
  for j in 0..args.max_count as usize {
    for k in 0..100usize {
      n6[j] += state0[j * 100 + k];
    }
  }
  n6[args.max_count as usize] = overmax;
  AkPullResult { n6 }
}

pub fn dp_avx2(args: AkPullArgs) -> AkPullResult {
  let n6 = vec![];
  AkPullResult { n6 }
}