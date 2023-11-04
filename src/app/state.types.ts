// import { event } from "@tauri-apps/api";

export type RenderParameters = {
    width: number;
    height: number;
    quality: number;
    computeParameters: ComputeParameters;
}

export type ComputeParameters = {
    finalVariation: Variation;
    postTransform: Affine;
    variations: Variation[];
}

export type LinearVariation = Variation;
export type SinusoidalVariation = Variation;
export type SphericalVariation = Variation;
export type SwirlVariation = Variation;
export type HorseshoeVariation = Variation;
export type PolarVariation = Variation;
export type HandkerchiefVariation = Variation;
export type HeartVariation = Variation;
export type DiscVariation = Variation;
export type SpiralVariation = Variation;
export type HyperbolicVariation = Variation;
export type DiamondVariation = Variation;
export type ExVariation = Variation;
export type JuliaVariation = Variation;
export type BentVariation = Variation;
export type WavesVariation = Variation;
export type FisheyeVariation = Variation;
export type PopcornVariation = Variation;
export type ExponentialVariation = Variation;
export type PowerVariation = Variation;
export type CosineVariation = Variation;
export type RingsVariation = Variation;
export type FanVariation = Variation;
export type BlobVariation = Variation<BlobParameters>;
export type PdjVariation = Variation<PDJParameters>;
export type Fan2Variation = Variation<Fan2Parameters>;
export type Rings2Variation = Variation<Rings2Parameters>;
export type EyefishVariation = Variation;
export type BubbleVariation = Variation;
export type CylinderVariation = Variation;
export type PerspectiveVariation = Variation<PerspectiveParameters>;
export type NoiseVariation = Variation;
export type JuliaNVariation = Variation<JuliaNParameters>;
export type JuliaScopeVariation = Variation<JuliaScopeParameters>;
export type BlurVariation = Variation;
export type GaussianBlurVariation = Variation;
export type RadialBlurVariation = Variation<RadialBlurParameters>;
export type PieVariation = Variation<PieParameters>;
export type NgonVariation = Variation<NgonParameters>;
export type CurlVariation = Variation<CurlParameters>;
export type RectanglesVariation = Variation<RectanglesParameters>;
export type ArchVariation = Variation;
export type TangentVariation = Variation;
export type SquareVariation = Variation;
export type RaysVariation = Variation;
export type BladeVariation = Variation;
export type SecantVariation = Variation;
export type TwintrianVariation = Variation;
export type CrossVariation = Variation;

export type Affine = {
    a: number;
    b: number;
    c: number;
    d: number;
    e: number;
    f: number;
}

export type Variation<ExtraParameters = {}> = ExtraParameters & {
    weight: number;
    affine: Affine;
    color: Color;
}

export type Color = {
    red: number;
    green: number;
    blue: number;
}

export type BlobParameters = {
    high: number;
    low: number;
    waves: number;
}

export type PDJParameters = {
    a: number;
    b: number;
    c: number;
    d: number;
}

export type Fan2Parameters = {
    x: number;
    y: number;
}

export type Rings2Parameters = {
    val: number;
}

export type PerspectiveParameters = {
    angle: number;
    dist: number;
}

export type JuliaNParameters = {
    power: number;
    dist: number;
}

export type JuliaScopeParameters = {
    power: number;
    dist: number;
}

export type RadialBlurParameters = {
    angle: number;
}

export type PieParameters = {
    slices: number;
    rotation: number;
    thickness: number;
}

export type NgonParameters = {
    power: number;
    sides: number;
    corners: number;
    circle: number;
}

export type CurlParameters = {
    c1: number;
    c2: number;
}

export type RectanglesParameters = {
    x: number;
    y: number;
}
