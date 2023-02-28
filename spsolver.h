#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int row;
    int col;
    double data;
} cdef_matrix;

typedef struct {
    int row;
    double data;
} cdef_vector;

typedef struct {
    int size;
    cdef_vector* data;
} cdef_ret_vector;

extern "C" __declspec(dllimport) cdef_ret_vector* spsolver_LU(cdef_matrix * A, int A_size, cdef_vector * B, int B_size, int row, int col);

#ifdef __cplusplus
}
#endif
