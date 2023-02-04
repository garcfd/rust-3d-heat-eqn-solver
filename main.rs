
// solve heat equation in 3d solver

extern crate ndarray;

use ndarray::Array3;

fn main() {

    let nits = 6000;
    const IM:usize = 101;
    const JM:usize = 101;
    const KM:usize = 101;

    let mut t = Array3::<f32>::zeros((IM,JM,KM));

//  set all values to zero
    for i in 0..IM {
    for j in 0..JM {
    for k in 0..KM {
      t[[i,j,k]] = 1.0;
    } } }

//  set inner values to zero 
    for i in 1..(IM-1) {
    for j in 1..(JM-1) {
    for k in 1..(KM-1) {
      t[[i,j,k]] = 0.0;
    } } }

    println!("started");

    for it in 1..=nits {


      for i in 1..IM-1 {
      for j in 1..JM-1 {
      for k in 1..KM-1 {
        t[[i,j,k]] = (t[[i-1,j,k]]+t[[i+1,j,k]]+t[[i,j-1,k]]+t[[i,j+1,k]]+t[[i,j,k-1]]+t[[i,j,k+1]])/6.0 ;
      } } }

      if it % 100 == 0 {
      println!("it={} t={}",it, t[[50,50,50]]);
      }

    }

    println!("finished");

}

