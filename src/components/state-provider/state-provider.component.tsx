import { ParentComponent, createContext, createEffect, useContext } from "solid-js";
import { createAppState } from "../../app";

export type StateProviderProps = {};

const StateContext = createContext<ReturnType<typeof createAppState>>(createAppState());

export const StateProvider: ParentComponent<StateProviderProps> = (props) => {
    const state = createAppState();
    createEffect(() => {
        console.warn(state.appState.projects.entities);
    })

    return (
        <StateContext.Provider value={state}>
            {props.children}
        </StateContext.Provider>
    );
};

export const useAppState = () => {
    const appState = useContext<ReturnType<typeof createAppState>>(
        StateContext as any,
    );
    return appState;
};
