import { SetStoreFunction } from "solid-js/store";
import { AppState } from "./state";
import { Affine, Color, ComputeParameters, Variation } from "./state.types";

type Setters<T> = {
    [K in keyof T as `set${Uppercase<string & K>}`]: (value: T[K]) => void;
};

export type VariationSetter<T extends Variation<any>> = {
    setWeight: (weight: number) => void;
    setAffine: (affine: Affine) => void;
    setColor: (color: Color) => void;
} & Setters<T>;

export const makeVariationSetter = <
    T extends Variation<Record<string, unknown>>,
>(
    variation: T,
    setVariation: (variation: T) => void,
): VariationSetter<T> => {
    return Object.keys(variation).reduce((accum, key) => {
        const setterName = `set${key[0].toUpperCase()}${key.slice(1)}`;
        return {
            ...accum,
            [setterName]: (value: T[typeof key]) =>
                setVariation({ ...variation, [key]: value }),
        };
    }, {} as VariationSetter<T>);
};

export const makeComputeParametersUpdates = (
    appState: AppState,
    setAppState: SetStoreFunction<AppState>,
) => {
    const computeParameters =
        appState.currentProject?.renderParameters.computeParameters;
    const setParameters =
        <K extends keyof ComputeParameters>(key: K) =>
        (params: ComputeParameters[K]) => {
            setAppState(
                "projects",
                "entities",
                appState.currentProjectId as keyof AppState["projects"]["entities"],
                "renderParameters",
                "computeParameters",
                key,
                params,
            );
        };
    if (!computeParameters) return {};
    return {
        linear: makeVariationSetter(
            computeParameters.linear,
            setParameters("linear"),
        ),
        sinusoidal: makeVariationSetter(
            computeParameters.sinusoidal,
            setParameters("sinusoidal"),
        ),
        spherical: makeVariationSetter(
            computeParameters.spherical,
            setParameters("spherical"),
        ),
        swirl: makeVariationSetter(
            computeParameters.swirl,
            setParameters("swirl"),
        ),
        horseshoe: makeVariationSetter(
            computeParameters.horseshoe,
            setParameters("horseshoe"),
        ),
        polar: makeVariationSetter(
            computeParameters.polar,
            setParameters("polar"),
        ),
        handkerchief: makeVariationSetter(
            computeParameters.handkerchief,
            setParameters("handkerchief"),
        ),
        heart: makeVariationSetter(
            computeParameters.heart,
            setParameters("heart"),
        ),
        disc: makeVariationSetter(
            computeParameters.disc,
            setParameters("disc"),
        ),
        spiral: makeVariationSetter(
            computeParameters.spiral,
            setParameters("spiral"),
        ),
        hyperbolic: makeVariationSetter(
            computeParameters.hyperbolic,
            setParameters("hyperbolic"),
        ),
        diamond: makeVariationSetter(
            computeParameters.diamond,
            setParameters("diamond"),
        ),
        ex: makeVariationSetter(computeParameters.ex, setParameters("ex")),
        julia: makeVariationSetter(
            computeParameters.julia,
            setParameters("julia"),
        ),
        bent: makeVariationSetter(
            computeParameters.bent,
            setParameters("bent"),
        ),
        waves: makeVariationSetter(
            computeParameters.waves,
            setParameters("waves"),
        ),
        fisheye: makeVariationSetter(
            computeParameters.fisheye,
            setParameters("fisheye"),
        ),
        popcorn: makeVariationSetter(
            computeParameters.popcorn,
            setParameters("popcorn"),
        ),
        exponential: makeVariationSetter(
            computeParameters.exponential,
            setParameters("exponential"),
        ),
        power: makeVariationSetter(
            computeParameters.power,
            setParameters("power"),
        ),
        cosine: makeVariationSetter(
            computeParameters.cosine,
            setParameters("cosine"),
        ),
        rings: makeVariationSetter(
            computeParameters.rings,
            setParameters("rings"),
        ),
        fan: makeVariationSetter(computeParameters.fan, setParameters("fan")),
        blob: makeVariationSetter(
            computeParameters.blob,
            setParameters("blob"),
        ),
        pdj: makeVariationSetter(computeParameters.pdj, setParameters("pdj")),
        fan2: makeVariationSetter(
            computeParameters.fan2,
            setParameters("fan2"),
        ),
        rings2: makeVariationSetter(
            computeParameters.rings2,
            setParameters("rings2"),
        ),
        eyefish: makeVariationSetter(
            computeParameters.eyefish,
            setParameters("eyefish"),
        ),
        bubble: makeVariationSetter(
            computeParameters.bubble,
            setParameters("bubble"),
        ),
        cylinder: makeVariationSetter(
            computeParameters.cylinder,
            setParameters("cylinder"),
        ),
        perspective: makeVariationSetter(
            computeParameters.perspective,
            setParameters("perspective"),
        ),
        noise: makeVariationSetter(
            computeParameters.noise,
            setParameters("noise"),
        ),
        juliaN: makeVariationSetter(
            computeParameters.juliaN,
            setParameters("juliaN"),
        ),
        juliaScope: makeVariationSetter(
            computeParameters.juliaScope,
            setParameters("juliaScope"),
        ),
        blur: makeVariationSetter(
            computeParameters.blur,
            setParameters("blur"),
        ),
        gaussianBlur: makeVariationSetter(
            computeParameters.gaussianBlur,
            setParameters("gaussianBlur"),
        ),
        radialBlur: makeVariationSetter(
            computeParameters.radialBlur,
            setParameters("radialBlur"),
        ),
        pie: makeVariationSetter(computeParameters.pie, setParameters("pie")),
        ngon: makeVariationSetter(
            computeParameters.ngon,
            setParameters("ngon"),
        ),
        curl: makeVariationSetter(
            computeParameters.curl,
            setParameters("curl"),
        ),
        rectangles: makeVariationSetter(
            computeParameters.rectangles,
            setParameters("rectangles"),
        ),
        arch: makeVariationSetter(
            computeParameters.arch,
            setParameters("arch"),
        ),
        tangent: makeVariationSetter(
            computeParameters.tangent,
            setParameters("tangent"),
        ),
        square: makeVariationSetter(
            computeParameters.square,
            setParameters("square"),
        ),
        rays: makeVariationSetter(
            computeParameters.rays,
            setParameters("rays"),
        ),
        blade: makeVariationSetter(
            computeParameters.blade,
            setParameters("blade"),
        ),
        secant: makeVariationSetter(
            computeParameters.secant,
            setParameters("secant"),
        ),
        twintrian: makeVariationSetter(
            computeParameters.twintrian,
            setParameters("twintrian"),
        ),
        cross: makeVariationSetter(
            computeParameters.cross,
            setParameters("cross"),
        ),
    };
};
