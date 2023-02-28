#include "spsolver.h"
#include <Eigen/Sparse>
#include <Eigen/SparseLU>

using namespace std;
using namespace Eigen;

cdef_ret_vector* spsolver_LU(cdef_matrix* A, int A_size, cdef_vector* B, int B_size, int row, int col) {
    // cdef_matrix* A -> SparseMatrix
    SparseMatrix<double> a(row, col);
    vector<Triplet<double>> tripletlist;
    for (int i = 0; i < A_size; i++) {
        int row = A[i].row;
        int col = A[i].col;
        tripletlist.push_back(Triplet<double>(row, col, A[i].data));
    }
    a.setFromTriplets(tripletlist.begin(), tripletlist.end());
    a.makeCompressed();

    // cdef_vector* B -> SparseVector
    SparseVector<double> b(row);
    for (int i = 0; i < B_size; i++) {
        int index = B[i].row;
        b.coeffRef(index) = B[i].data;
    }

    // Solver.
    SparseLU<SparseMatrix<double>> solver_sparse;
    solver_sparse.compute(a);
    SparseVector<double> x = solver_sparse.solve(b);

    // Prepare parameter cdef_ret_vector for returning.
    cdef_ret_vector* ret = (cdef_ret_vector*)calloc(1, sizeof(cdef_ret_vector));
    cdef_vector* cdef_x = (cdef_vector*)calloc(x.size(), sizeof(cdef_vector));

    // If calloc function fails, should return a zero size cdef_ret_vetor.
    // It would sometimes occur when the memory space is not enough.
    if (cdef_x == NULL) {
        ret[0].size = 0;
        ret[0].data = NULL;

    }
    else {
        int x_idx = 0;
        for (SparseVector<double>::InnerIterator it(x); it; ++it) {
            cdef_vector foo;
            foo.row = it.index();
            foo.data = it.value();
            cdef_x[x_idx] = foo;
            x_idx++;
        }
        ret[0].size = x_idx;
        ret[0].data = cdef_x;
    }

    return ret;
}