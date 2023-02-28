use sprs::CsMat;

mod lib_loading_wrapper;

fn main() {
    println!("Hello, world!");
    let a: CsMat<f64> = CsMat::eye(3);
    let b: CsMat<f64> = CsMat::eye(3);
    let c = lib_loading_wrapper::spsolver_LU(&a, &b);
    println!("{:?}", c);
}
