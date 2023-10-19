import { event } from "@tauri-apps/api";

export type RenderParameters = {
    width: number;
    height: number;
    quality: number;
    computeParameters: ComputeParameters;
}

export type ComputeParameters = {
    // finalVariation: FinalVariation | null;
    postTransform: Affine;
    linear: Variation;
    sinusoidal: Variation;
    spherical: Variation;
    swirl: Variation;
    horseshoe: Variation;
    polar: Variation;
    handkerchief: Variation;
    heart: Variation;
    disc: Variation;
    spiral: Variation;
    hyperbolic: Variation;
    diamond: Variation;
    ex: Variation;
    julia: Variation;
    bent: Variation;
    waves: Variation;
    fisheye: Variation;
    popcorn: Variation;
    exponential: Variation;
    power: Variation;
    cosine: Variation;
    rings: Variation;
    fan: Variation;
    blob: Variation<BlobParameters>;
    pdj: Variation<PDJParameters>;
    fan2: Variation<Fan2Parameters>;
    rings2: Variation<Rings2Parameters>;
    eyefish: Variation;
    bubble: Variation;
    cylinder: Variation;
    perspective: Variation<PerspectiveParameters>;
    noise: Variation;
    juliaN: Variation<JuliaNParameters>;
    juliaScope: Variation<JuliaScopeParameters>;
    blur: Variation;
    gaussianBlur: Variation;
    radialBlur: Variation<RadialBlurParameters>;
    pie: Variation<PieParameters>;
    ngon: Variation<NgonParameters>;
    curl: Variation<CurlParameters>;
    rectangles: Variation<RectanglesParameters>;
    arch: Variation;
    tangent: Variation;
    square: Variation;
    rays: Variation;
    blade: Variation;
    secant: Variation;
    twintrian: Variation;
    cross: Variation;



}

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
