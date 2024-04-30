//use cargo bench to test
//去集成刘迪的库，刘迪发布一下自己的库
//use baby-plonk-rust::*;

#![allow(unused)]
fn main() {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn verify_proof_test() {
        todo!()
    }
    //此处先借用刘迪baby-plonk-rust库中的test代码
    // fn verify_proof_test() {
    //     //从1到tau^7*g1, tau = 1
    //     let setup = Setup::generate_srs(8 + 6, Scalar::from(101));
    
    //     //程序为：
    //     //c <== a * b
    //     //b <== a + d
    //     let constraints: Vec<_> = vec!["e public", "c <== a * b + b", "e <== c * d"]
    //         .into_iter()
    //         .map(AssemblyEqn::eq_to_assembly)
    //         .collect();
    
    //     let program = Program::new(constraints, 8);
    
    //     let mut prover = Prover::new(setup.clone(), program.clone());
    
    //     let mut witness: HashMap<String, Scalar> = HashMap::new();
    //     witness.insert("a".to_owned(), Scalar::from(3));
    //     witness.insert("b".to_owned(), Scalar::from(4));
    //     witness.insert("c".to_owned(), Scalar::from(16));
    //     witness.insert("d".to_owned(), Scalar::from(5));
    //     witness.insert("e".to_owned(), Scalar::from(80));
    
    //     let prove_start = Instant::now();
    //     let proof = prover.prove(witness);
    //     let prove_duration = prove_start.elapsed();
    
    //     let mut verifier = Verifier::new(setup, program, proof);
    
    //     let verify_start = Instant::now();
    //     assert_eq!(verifier.verify(vec![Scalar::from(80)]), true);
    //     let verify_duration = verify_start.elapsed();
    
    //     println!("Verification passed");
    //     println!("Prove time: {:?}", prove_duration);
    //     println!("Verify time: {:?}", verify_duration);
    // }

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("verfiy", |b| b.iter(|| verify_proof_test()));
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);
}
