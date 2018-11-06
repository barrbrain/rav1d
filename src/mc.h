/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef __DAV1D_SRC_MC_H__
#define __DAV1D_SRC_MC_H__

#include <stdint.h>
#include <stddef.h>

#include "common/bitdepth.h"

#include "src/levels.h"

#define decl_mc_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            int w, int h, int mx, int my)
typedef decl_mc_fn(*mc_fn);

#define decl_warp8x8_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            const int16_t *abcd, int mx, int my)
typedef decl_warp8x8_fn(*warp8x8_fn);

#define decl_mct_fn(name) \
void (name)(coef *tmp, const pixel *src, ptrdiff_t src_stride, \
            int w, int h, int mx, int my)
typedef decl_mct_fn(*mct_fn);

#define decl_warp8x8t_fn(name) \
void (name)(coef *tmp, const ptrdiff_t tmp_stride, \
            const pixel *src, ptrdiff_t src_stride, \
            const int16_t *abcd, int mx, int my)
typedef decl_warp8x8t_fn(*warp8x8t_fn);

#define decl_avg_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const coef *tmp1, const coef *tmp2, int w, int h)
typedef decl_avg_fn(*avg_fn);

#define decl_w_avg_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const coef *tmp1, const coef *tmp2, int w, int h, int weight)
typedef decl_w_avg_fn(*w_avg_fn);

#define decl_mask_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const coef *tmp1, const coef *tmp2, int w, int h, \
            const uint8_t *mask)
typedef decl_mask_fn(*mask_fn);

#define decl_w_mask_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const coef *tmp1, const coef *tmp2, int w, int h, \
            uint8_t *mask, int sign)
typedef decl_w_mask_fn(*w_mask_fn);

#define decl_blend_fn(name) \
void (name)(pixel *dst, ptrdiff_t dst_stride, \
            const pixel *tmp, int w, int h, \
            const uint8_t *mask, ptrdiff_t mstride)
typedef decl_blend_fn(*blend_fn);

#define decl_emu_edge_fn(name) \
void (name)(intptr_t bw, intptr_t bh, intptr_t iw, intptr_t ih, intptr_t x, intptr_t y, \
            pixel *dst, ptrdiff_t dst_stride, const pixel *src, ptrdiff_t src_stride)
typedef decl_emu_edge_fn(*emu_edge_fn);

typedef struct Dav1dMCDSPContext {
    mc_fn mc[N_2D_FILTERS];
    mct_fn mct[N_2D_FILTERS];
    avg_fn avg;
    w_avg_fn w_avg;
    mask_fn mask;
    w_mask_fn w_mask[3 /* 444, 422, 420 */];
    blend_fn blend;
    warp8x8_fn warp8x8;
    warp8x8t_fn warp8x8t;
    emu_edge_fn emu_edge;
} Dav1dMCDSPContext;

void dav1d_mc_dsp_init_8bpc(Dav1dMCDSPContext *c);
void dav1d_mc_dsp_init_10bpc(Dav1dMCDSPContext *c);

void dav1d_mc_dsp_init_arm_8bpc(Dav1dMCDSPContext *c);
void dav1d_mc_dsp_init_arm_10bpc(Dav1dMCDSPContext *c);

void dav1d_mc_dsp_init_x86_8bpc(Dav1dMCDSPContext *c);
void dav1d_mc_dsp_init_x86_10bpc(Dav1dMCDSPContext *c);

#endif /* __DAV1D_SRC_MC_H__ */
