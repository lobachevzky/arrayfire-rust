#![doc(html_logo_url = "http://www.arrayfire.com/logos/arrayfire_logo_symbol.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://arrayfire.com/docs/rust")]

#[macro_use]
extern crate lazy_static;

pub use array::Array;
pub use array::{print, print_gen, eval_multiple, is_eval_manual, set_manual_eval};
mod array;

//pub use algorithm::{sum_nan, product_nan, sum_nan_all, product_nan_all};
pub use algorithm::{sum, product, min, max, all_true, any_true, count, sum_nan, product_nan};
pub use algorithm::{sum_all, product_all, min_all, max_all, sum_nan_all, product_nan_all};
pub use algorithm::{all_true_all, any_true_all, count_all, imin, imax, imin_all, imax_all};
pub use algorithm::{accum, locate, diff1, diff2, sort, sort_index, sort_by_key};
pub use algorithm::{set_unique, set_union, set_intersect, scan, scan_by_key};
mod algorithm;

pub use arith::{add, sub, div, mul, lt, gt, le, ge, eq, neq, and, or, minof, maxof, rem};
pub use arith::{bitand, bitor, bitxor, shiftl, shiftr};
pub use arith::{abs, sign, round, trunc, floor, ceil, modulo, sigmoid, clamp};
pub use arith::{sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh};
pub use arith::{atan2, cplx2, arg, cplx, real, imag, conjg, hypot};
pub use arith::{sqrt, log, log1p, log10, log2, pow2, exp, expm1, erf, erfc, root, pow};
pub use arith::{cbrt, factorial, tgamma, lgamma, iszero, isinf, isnan};
mod arith;

pub use backend::{set_backend, get_backend_count, get_available_backends, get_active_backend};
mod backend;

pub use blas::{matmul, dot, transpose, transpose_inplace};
mod blas;

pub use data::{constant, range, iota, identity};
pub use data::{diag_create, diag_extract, lower, upper};
pub use data::{join, join_many, tile};
pub use data::{reorder, shift, moddims, flat, flip};
pub use data::{select, selectl, selectr, replace, replace_scalar};
pub use data::{range_t, iota_t, identity_t, constant_t};
mod data;

pub use device::{get_version, get_revision, info, info_string, device_info, init, device_count, is_double_available, set_device, get_device};
pub use device::{device_mem_info, print_mem_info, set_mem_step_size, get_mem_step_size, device_gc, sync};
mod device;

pub use defines::{DType, AfError, Backend, ColorMap, YCCStd, HomographyType};
pub use defines::{InterpType, BorderType, MatchType, NormType};
pub use defines::{Connectivity, ConvMode, ConvDomain, ColorSpace, MatProp};
pub use defines::{MarkerType, MomentType, SparseFormat, BinaryOp, RandomEngineType};
pub use defines::{PHILOX, THREEFRY, MERSENNE, DEFAULT_RANDOM_ENGINE, Scalar};
mod defines;

pub use dim4::Dim4;
mod dim4;

pub use error::{Callback, ErrorCallback, register_error_handler, handle_error_general};
mod error;

pub use index::{Indexer, index, row, rows, col, cols, slice, slices,
                set_row, set_rows, set_col, set_cols, set_slice, set_slices,
                lookup, assign_seq, index_gen, assign_gen};
mod index;

pub use seq::Seq;
mod seq;

pub use graphics::Window;
mod graphics;

pub use image::{gaussian_kernel, load_image, load_image_native, save_image, save_image_native};
pub use image::{resize, transform, rotate, translate, scale, skew};
pub use image::{dilate, dilate3, erode, erode3, minfilt, maxfilt};
pub use image::{gradient, histogram, hist_equal, regions};
pub use image::{gray2rgb, rgb2gray, hsv2rgb, rgb2hsv, color_space};
pub use image::{bilateral, mean_shift, medfilt, sobel, medfilt1};
pub use image::{unwrap, wrap, sat, rgb2ycbcr, ycbcr2rgb, is_imageio_available, transform_coords};
pub use image::{moments, moments_all};
mod image;

pub use lapack::{svd, lu, qr, cholesky, solve, solve_lu, inverse, det, rank, norm};
pub use lapack::{svd_inplace, lu_inplace, qr_inplace, cholesky_inplace, is_lapack_available};
mod lapack;
mod macros;
mod num;

pub use random::RandomEngine;
pub use random::{set_seed, get_seed, randu, randn, random_uniform, random_normal};
pub use random::{get_default_random_engine, set_default_random_engine_type};
mod random;

pub use signal::{approx1, approx2, set_fft_plan_cache_size};
pub use signal::{fft, fft2, fft3, ifft, ifft2, ifft3};
pub use signal::{fft_r2c, fft2_r2c, fft3_r2c, fft_c2r, fft2_c2r, fft3_c2r};
pub use signal::{fft_inplace, fft2_inplace, fft3_inplace};
pub use signal::{ifft_inplace, ifft2_inplace, ifft3_inplace};
pub use signal::{convolve1, convolve2, convolve3, convolve2_sep};
pub use signal::{fft_convolve1, fft_convolve2, fft_convolve3};
pub use signal::{fir, iir};
mod signal;

pub use sparse::{sparse, sparse_from_host, sparse_from_dense, sparse_convert_to};
pub use sparse::{sparse_to_dense, sparse_get_info, sparse_get_values, sparse_get_nnz};
pub use sparse::{sparse_get_row_indices, sparse_get_col_indices, sparse_get_format};
mod sparse;

pub use statistics::{mean, stdev, median, var, cov, corrcoef};
pub use statistics::{mean_weighted, var_weighted};
pub use statistics::{var_all, mean_all, stdev_all, median_all};
pub use statistics::{mean_all_weighted, var_all_weighted};
mod statistics;

pub use util::{HasAfEnum, get_size};
mod util;

pub use vision::Features;
pub use vision::{fast, harris, orb, hamming_matcher, nearest_neighbour, match_template, susan, dog};
pub use vision::{homography};
mod vision;

// headers that are not exposed through rust wrapper are as follows:
// compatible.h
// constants.h
// complex.h
// cuda.h
// opencl.h
