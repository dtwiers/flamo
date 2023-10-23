import { StoreSetter } from "solid-js/store";
import { AppState } from "./state";
import { Affine, Color, Variation } from "./state.types";

type Setters<T> = {
    [K in keyof T as `set${Uppercase<string & K>}`]: (value: T[K]) => void;
};

export type VariationSetter<T extends Variation<any>> = {
    setWeight: (weight: number) => void;
    setAffine: (affine: Affine) => void;
    setColor: (color: Color) => void;

} & Setters<T>;

export const makeVariationSetter = <T extends Variation<any>>(
    variation: T,
    setVariation: (variation: T) => void
): VariationSetter<T> => {
    Object.keys(variation).map((key) -> {
        const setterName = `set${key[0].toUpperCase()}${key.slice(1)}`;
        return {
            [setterName]: (value: T[K]) => setVariation({ ...variation, [key]: value })
        };
    })
    return {
        setWeight: (weight) => setVariation({ ...variation, weight }),
        setAffine: (affine) => setVariation({ ...variation, affine }),
        setColor: (color) => setVariation({ ...variation, color }),
    } as VariationSetter<T>;
};

export const makeComputeParametersUpdates = (
    appState: AppState,
    setAppState: StoreSetter<AppState>
) => {
    return {
        
    };
};
