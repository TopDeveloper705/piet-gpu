// SPDX-License-Identifier: Apache-2.0 OR MIT OR Unlicense

// Code auto-generated by piet-gpu-derive

struct LineSegRef {
    uint offset;
};

struct QuadSegRef {
    uint offset;
};

struct CubicSegRef {
    uint offset;
};

struct FillRef {
    uint offset;
};

struct FillImageRef {
    uint offset;
};

struct StrokeRef {
    uint offset;
};

struct SetLineWidthRef {
    uint offset;
};

struct TransformRef {
    uint offset;
};

struct ClipRef {
    uint offset;
};

struct ElementRef {
    uint offset;
};

struct LineSeg {
    vec2 p0;
    vec2 p1;
};

#define LineSeg_size 16

LineSegRef LineSeg_index(LineSegRef ref, uint index) {
    return LineSegRef(ref.offset + index * LineSeg_size);
}

struct QuadSeg {
    vec2 p0;
    vec2 p1;
    vec2 p2;
};

#define QuadSeg_size 24

QuadSegRef QuadSeg_index(QuadSegRef ref, uint index) {
    return QuadSegRef(ref.offset + index * QuadSeg_size);
}

struct CubicSeg {
    vec2 p0;
    vec2 p1;
    vec2 p2;
    vec2 p3;
};

#define CubicSeg_size 32

CubicSegRef CubicSeg_index(CubicSegRef ref, uint index) {
    return CubicSegRef(ref.offset + index * CubicSeg_size);
}

struct Fill {
    uint rgba_color;
};

#define Fill_size 4

FillRef Fill_index(FillRef ref, uint index) {
    return FillRef(ref.offset + index * Fill_size);
}

struct FillImage {
    uint index;
    ivec2 offset;
};

#define FillImage_size 8

FillImageRef FillImage_index(FillImageRef ref, uint index) {
    return FillImageRef(ref.offset + index * FillImage_size);
}

struct Stroke {
    uint rgba_color;
};

#define Stroke_size 4

StrokeRef Stroke_index(StrokeRef ref, uint index) {
    return StrokeRef(ref.offset + index * Stroke_size);
}

struct SetLineWidth {
    float width;
};

#define SetLineWidth_size 4

SetLineWidthRef SetLineWidth_index(SetLineWidthRef ref, uint index) {
    return SetLineWidthRef(ref.offset + index * SetLineWidth_size);
}

struct Transform {
    vec4 mat;
    vec2 translate;
};

#define Transform_size 24

TransformRef Transform_index(TransformRef ref, uint index) {
    return TransformRef(ref.offset + index * Transform_size);
}

struct Clip {
    vec4 bbox;
};

#define Clip_size 16

ClipRef Clip_index(ClipRef ref, uint index) {
    return ClipRef(ref.offset + index * Clip_size);
}

#define Element_Nop 0
#define Element_StrokeLine 1
#define Element_FillLine 2
#define Element_StrokeQuad 3
#define Element_FillQuad 4
#define Element_StrokeCubic 5
#define Element_FillCubic 6
#define Element_Stroke 7
#define Element_Fill 8
#define Element_SetLineWidth 9
#define Element_Transform 10
#define Element_BeginClip 11
#define Element_EndClip 12
#define Element_FillImage 13
#define Element_size 36

ElementRef Element_index(ElementRef ref, uint index) {
    return ElementRef(ref.offset + index * Element_size);
}

struct ElementTag {
   uint tag;
   uint flags;
};

LineSeg LineSeg_read(LineSegRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    uint raw1 = scene[ix + 1];
    uint raw2 = scene[ix + 2];
    uint raw3 = scene[ix + 3];
    LineSeg s;
    s.p0 = vec2(uintBitsToFloat(raw0), uintBitsToFloat(raw1));
    s.p1 = vec2(uintBitsToFloat(raw2), uintBitsToFloat(raw3));
    return s;
}

QuadSeg QuadSeg_read(QuadSegRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    uint raw1 = scene[ix + 1];
    uint raw2 = scene[ix + 2];
    uint raw3 = scene[ix + 3];
    uint raw4 = scene[ix + 4];
    uint raw5 = scene[ix + 5];
    QuadSeg s;
    s.p0 = vec2(uintBitsToFloat(raw0), uintBitsToFloat(raw1));
    s.p1 = vec2(uintBitsToFloat(raw2), uintBitsToFloat(raw3));
    s.p2 = vec2(uintBitsToFloat(raw4), uintBitsToFloat(raw5));
    return s;
}

CubicSeg CubicSeg_read(CubicSegRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    uint raw1 = scene[ix + 1];
    uint raw2 = scene[ix + 2];
    uint raw3 = scene[ix + 3];
    uint raw4 = scene[ix + 4];
    uint raw5 = scene[ix + 5];
    uint raw6 = scene[ix + 6];
    uint raw7 = scene[ix + 7];
    CubicSeg s;
    s.p0 = vec2(uintBitsToFloat(raw0), uintBitsToFloat(raw1));
    s.p1 = vec2(uintBitsToFloat(raw2), uintBitsToFloat(raw3));
    s.p2 = vec2(uintBitsToFloat(raw4), uintBitsToFloat(raw5));
    s.p3 = vec2(uintBitsToFloat(raw6), uintBitsToFloat(raw7));
    return s;
}

Fill Fill_read(FillRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    Fill s;
    s.rgba_color = raw0;
    return s;
}

FillImage FillImage_read(FillImageRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    uint raw1 = scene[ix + 1];
    FillImage s;
    s.index = raw0;
    s.offset = ivec2(int(raw1 << 16) >> 16, int(raw1) >> 16);
    return s;
}

Stroke Stroke_read(StrokeRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    Stroke s;
    s.rgba_color = raw0;
    return s;
}

SetLineWidth SetLineWidth_read(SetLineWidthRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    SetLineWidth s;
    s.width = uintBitsToFloat(raw0);
    return s;
}

Transform Transform_read(TransformRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    uint raw1 = scene[ix + 1];
    uint raw2 = scene[ix + 2];
    uint raw3 = scene[ix + 3];
    uint raw4 = scene[ix + 4];
    uint raw5 = scene[ix + 5];
    Transform s;
    s.mat = vec4(uintBitsToFloat(raw0), uintBitsToFloat(raw1), uintBitsToFloat(raw2), uintBitsToFloat(raw3));
    s.translate = vec2(uintBitsToFloat(raw4), uintBitsToFloat(raw5));
    return s;
}

Clip Clip_read(ClipRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = scene[ix + 0];
    uint raw1 = scene[ix + 1];
    uint raw2 = scene[ix + 2];
    uint raw3 = scene[ix + 3];
    Clip s;
    s.bbox = vec4(uintBitsToFloat(raw0), uintBitsToFloat(raw1), uintBitsToFloat(raw2), uintBitsToFloat(raw3));
    return s;
}

ElementTag Element_tag(ElementRef ref) {
    uint tag_and_flags = scene[ref.offset >> 2];
    return ElementTag(tag_and_flags & 0xffff, tag_and_flags >> 16);
}

LineSeg Element_StrokeLine_read(ElementRef ref) {
    return LineSeg_read(LineSegRef(ref.offset + 4));
}

LineSeg Element_FillLine_read(ElementRef ref) {
    return LineSeg_read(LineSegRef(ref.offset + 4));
}

QuadSeg Element_StrokeQuad_read(ElementRef ref) {
    return QuadSeg_read(QuadSegRef(ref.offset + 4));
}

QuadSeg Element_FillQuad_read(ElementRef ref) {
    return QuadSeg_read(QuadSegRef(ref.offset + 4));
}

CubicSeg Element_StrokeCubic_read(ElementRef ref) {
    return CubicSeg_read(CubicSegRef(ref.offset + 4));
}

CubicSeg Element_FillCubic_read(ElementRef ref) {
    return CubicSeg_read(CubicSegRef(ref.offset + 4));
}

Stroke Element_Stroke_read(ElementRef ref) {
    return Stroke_read(StrokeRef(ref.offset + 4));
}

Fill Element_Fill_read(ElementRef ref) {
    return Fill_read(FillRef(ref.offset + 4));
}

SetLineWidth Element_SetLineWidth_read(ElementRef ref) {
    return SetLineWidth_read(SetLineWidthRef(ref.offset + 4));
}

Transform Element_Transform_read(ElementRef ref) {
    return Transform_read(TransformRef(ref.offset + 4));
}

Clip Element_BeginClip_read(ElementRef ref) {
    return Clip_read(ClipRef(ref.offset + 4));
}

Clip Element_EndClip_read(ElementRef ref) {
    return Clip_read(ClipRef(ref.offset + 4));
}

FillImage Element_FillImage_read(ElementRef ref) {
    return FillImage_read(FillImageRef(ref.offset + 4));
}

