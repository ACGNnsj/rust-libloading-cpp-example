use std::path::{Path,PathBuf};
use libloading::{Library, Symbol};
use sprs::{TriMat, CsMat};

type SPSolverLU = unsafe fn(*const cdef_matrix, i32, *const cdef_vector, i32, i32, i32) -> *mut cdef_ret_vector;

#[repr(C)]
pub struct cdef_matrix {
    row: i32,
    col: i32,
    data: f64,
}

#[repr(C)]
#[derive(Debug)]
pub struct cdef_vector {
    row: i32,
    data: f64,
}

#[repr(C)]
pub struct cdef_ret_vector {
    size: i32,
    data: *mut cdef_vector,
}

pub fn spsolver_LU(a: &CsMat<f64>, b: &CsMat<f64>) -> CsMat<f64> {
    let path=Path::new("./cmake-build-release/Release/spsolver.dll");
    println!("{:?}",path.canonicalize());
    let path_buf=path.canonicalize().unwrap();
    println!("{:?}",path_buf);
    let my_path_buf=PathBuf::new();
    println!("{:?}",my_path_buf);
    println!("\\\\?\\G:\\clion\\testExternC\\cmake-build-release\\Release\\spsolver.dll");
    let lib = unsafe { Library::new(path.canonicalize().unwrap()).unwrap() };
    let rows = a.rows() as i32;
    let cols = a.cols() as i32;
    let a = sparse_to_cdef_matrix(a);
    let b = sparse_to_cdef_vector(b);
    let a_len = a.len() as i32;
    let b_len = b.len() as i32;
    unsafe {
        let func: Symbol<SPSolverLU> = lib.get(b"spsolver_LU").unwrap();
        let x = func(a.as_ptr(), a_len, b.as_ptr(), b_len, rows, cols);
        // rebuild results as Vec<cdef_veoctr>
        let x_len = (*x).size as usize;
        let x_cap = x_len;
        let x_ptr = (*x).data;
        let rebuilt_x = Vec::from_raw_parts(x_ptr, x_len, x_cap);
        cdef_vector_to_sparse(rebuilt_x, rows as usize, cols as usize)
    }
}

fn sparse_to_cdef_matrix(x: &CsMat<f64>) -> Vec<cdef_matrix> {
    let mut ret = vec![];
    for (&v, (row, col)) in x.iter() {
        ret.push(cdef_matrix { row: row as i32, col: col as i32, data: v });
    }

    ret
}

fn sparse_to_cdef_vector(x: &CsMat<f64>) -> Vec<cdef_vector> {
    let mut ret = vec![];
    for (&v, (row, _)) in x.iter() {
        ret.push(cdef_vector { row: row as i32, data: v });
    }

    ret
}

fn cdef_vector_to_sparse(x: Vec<cdef_vector>, nrows: usize, ncols: usize) -> CsMat<f64> {
    let mut tri = TriMat::new((nrows, 1));
    for vector in x.iter() {
        tri.add_triplet(vector.row as usize, 0, vector.data);
    }
    tri.to_csc()
}