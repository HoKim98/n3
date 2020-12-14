mod base;
mod cpu;
mod cuda;

pub use self::base::CandidatesMachine;
pub use self::cpu::CpuMachine;
pub use self::cuda::CudaMachine;
