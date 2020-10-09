// Code auto-generated by piet-gpu-derive

struct AnnoFillRef {
    uint offset;
};

struct AnnoFillMaskRef {
    uint offset;
};

struct AnnoStrokeRef {
    uint offset;
};

struct AnnotatedRef {
    uint offset;
};

struct AnnoFill {
    uint rgba_color;
    vec4 bbox;
};

#define AnnoFill_size 20

AnnoFillRef AnnoFill_index(AnnoFillRef ref, uint index) {
    return AnnoFillRef(ref.offset + index * AnnoFill_size);
}

struct AnnoFillMask {
    float mask;
    vec4 bbox;
};

#define AnnoFillMask_size 20

AnnoFillMaskRef AnnoFillMask_index(AnnoFillMaskRef ref, uint index) {
    return AnnoFillMaskRef(ref.offset + index * AnnoFillMask_size);
}

struct AnnoStroke {
    uint rgba_color;
    vec4 bbox;
    float linewidth;
};

#define AnnoStroke_size 24

AnnoStrokeRef AnnoStroke_index(AnnoStrokeRef ref, uint index) {
    return AnnoStrokeRef(ref.offset + index * AnnoStroke_size);
}

#define Annotated_Nop 0
#define Annotated_Stroke 1
#define Annotated_Fill 2
#define Annotated_FillMask 3
#define Annotated_FillMaskInv 4
#define Annotated_size 28

AnnotatedRef Annotated_index(AnnotatedRef ref, uint index) {
    return AnnotatedRef(ref.offset + index * Annotated_size);
}

AnnoFill AnnoFill_read(AnnoFillRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = annotated[ix + 0];
    uint raw1 = annotated[ix + 1];
    uint raw2 = annotated[ix + 2];
    uint raw3 = annotated[ix + 3];
    uint raw4 = annotated[ix + 4];
    AnnoFill s;
    s.rgba_color = raw0;
    s.bbox = vec4(uintBitsToFloat(raw1), uintBitsToFloat(raw2), uintBitsToFloat(raw3), uintBitsToFloat(raw4));
    return s;
}

void AnnoFill_write(AnnoFillRef ref, AnnoFill s) {
    uint ix = ref.offset >> 2;
    annotated[ix + 0] = s.rgba_color;
    annotated[ix + 1] = floatBitsToUint(s.bbox.x);
    annotated[ix + 2] = floatBitsToUint(s.bbox.y);
    annotated[ix + 3] = floatBitsToUint(s.bbox.z);
    annotated[ix + 4] = floatBitsToUint(s.bbox.w);
}

AnnoFillMask AnnoFillMask_read(AnnoFillMaskRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = annotated[ix + 0];
    uint raw1 = annotated[ix + 1];
    uint raw2 = annotated[ix + 2];
    uint raw3 = annotated[ix + 3];
    uint raw4 = annotated[ix + 4];
    AnnoFillMask s;
    s.mask = uintBitsToFloat(raw0);
    s.bbox = vec4(uintBitsToFloat(raw1), uintBitsToFloat(raw2), uintBitsToFloat(raw3), uintBitsToFloat(raw4));
    return s;
}

void AnnoFillMask_write(AnnoFillMaskRef ref, AnnoFillMask s) {
    uint ix = ref.offset >> 2;
    annotated[ix + 0] = floatBitsToUint(s.mask);
    annotated[ix + 1] = floatBitsToUint(s.bbox.x);
    annotated[ix + 2] = floatBitsToUint(s.bbox.y);
    annotated[ix + 3] = floatBitsToUint(s.bbox.z);
    annotated[ix + 4] = floatBitsToUint(s.bbox.w);
}

AnnoStroke AnnoStroke_read(AnnoStrokeRef ref) {
    uint ix = ref.offset >> 2;
    uint raw0 = annotated[ix + 0];
    uint raw1 = annotated[ix + 1];
    uint raw2 = annotated[ix + 2];
    uint raw3 = annotated[ix + 3];
    uint raw4 = annotated[ix + 4];
    uint raw5 = annotated[ix + 5];
    AnnoStroke s;
    s.rgba_color = raw0;
    s.bbox = vec4(uintBitsToFloat(raw1), uintBitsToFloat(raw2), uintBitsToFloat(raw3), uintBitsToFloat(raw4));
    s.linewidth = uintBitsToFloat(raw5);
    return s;
}

void AnnoStroke_write(AnnoStrokeRef ref, AnnoStroke s) {
    uint ix = ref.offset >> 2;
    annotated[ix + 0] = s.rgba_color;
    annotated[ix + 1] = floatBitsToUint(s.bbox.x);
    annotated[ix + 2] = floatBitsToUint(s.bbox.y);
    annotated[ix + 3] = floatBitsToUint(s.bbox.z);
    annotated[ix + 4] = floatBitsToUint(s.bbox.w);
    annotated[ix + 5] = floatBitsToUint(s.linewidth);
}

uint Annotated_tag(AnnotatedRef ref) {
    return annotated[ref.offset >> 2];
}

AnnoStroke Annotated_Stroke_read(AnnotatedRef ref) {
    return AnnoStroke_read(AnnoStrokeRef(ref.offset + 4));
}

AnnoFill Annotated_Fill_read(AnnotatedRef ref) {
    return AnnoFill_read(AnnoFillRef(ref.offset + 4));
}

AnnoFillMask Annotated_FillMask_read(AnnotatedRef ref) {
    return AnnoFillMask_read(AnnoFillMaskRef(ref.offset + 4));
}

AnnoFillMask Annotated_FillMaskInv_read(AnnotatedRef ref) {
    return AnnoFillMask_read(AnnoFillMaskRef(ref.offset + 4));
}

void Annotated_Nop_write(AnnotatedRef ref) {
    annotated[ref.offset >> 2] = Annotated_Nop;
}

void Annotated_Stroke_write(AnnotatedRef ref, AnnoStroke s) {
    annotated[ref.offset >> 2] = Annotated_Stroke;
    AnnoStroke_write(AnnoStrokeRef(ref.offset + 4), s);
}

void Annotated_Fill_write(AnnotatedRef ref, AnnoFill s) {
    annotated[ref.offset >> 2] = Annotated_Fill;
    AnnoFill_write(AnnoFillRef(ref.offset + 4), s);
}

void Annotated_FillMask_write(AnnotatedRef ref, AnnoFillMask s) {
    annotated[ref.offset >> 2] = Annotated_FillMask;
    AnnoFillMask_write(AnnoFillMaskRef(ref.offset + 4), s);
}

void Annotated_FillMaskInv_write(AnnotatedRef ref, AnnoFillMask s) {
    annotated[ref.offset >> 2] = Annotated_FillMaskInv;
    AnnoFillMask_write(AnnoFillMaskRef(ref.offset + 4), s);
}

