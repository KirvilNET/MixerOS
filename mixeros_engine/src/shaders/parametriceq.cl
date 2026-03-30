// MixerOS Parametric EQ
typedef struct {
    float b0, b1, b2, a1, a2;
} EqBand;

// coeffs.bands is laid out as: [b0, b1, b2, a1, a2,  b0, b1, b2, a1, a2, ...]
//                               |---- band 0 ----|   |---- band 1 ----|
void get_band(
    __constant const float* bands,
    uint i,
    EqBand* out)
{
    uint base = i * 5;
    out->b0 = bands[base + 0];
    out->b1 = bands[base + 1];
    out->b2 = bands[base + 2];
    out->a1 = bands[base + 3];
    out->a2 = bands[base + 4];
}

__kernel void eq_process(
    __global const float* input_data,
    __global       float* output_data,
    __constant     float* bands,       // flattened [num_bands * 5] coefficients
    uint                  num_samples, // samples per channel
    uint                  num_bands)   // number of EQ bands (replaces the hardcoded 6)
{
    uint channel = get_global_id(0);

    // Per-channel biquad state — one w1/w2 per band.
    // OpenCL C requires fixed-size arrays; size must be a compile-time constant.
    // 6 bands is the max here; guard with num_bands at runtime.
    float w1[6] = {0, 0, 0, 0, 0, 0};
    float w2[6] = {0, 0, 0, 0, 0, 0};

    uint base = channel * num_samples;

    for (uint i = 0; i < num_samples; i++) {
        float x = input_data[base + i];

        for (uint band = 0; band < num_bands; band++) {
            EqBand b;
            get_band(bands, band, &b);

            // Transposed Direct Form II biquad
            float w = x - (b.a1 * w1[band]) - (b.a2 * w2[band]);
            float y = (b.b0 * w) + (b.b1 * w1[band]) + (b.b2 * w2[band]);

            w2[band] = w1[band];
            w1[band] = w;
            x = y;
        }

        output_data[base + i] = x;
    }
}