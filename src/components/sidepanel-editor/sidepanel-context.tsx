import { Accessor, createContext, createSignal, useContext } from "solid-js";

export type SidepanelContext = readonly [
    Accessor<string | null>,
    (sidepanel: string | null) => void,
];

const SidepanelContext = createContext<SidepanelContext>([() => null, () => {
    console.error("SidepanelContext not initialized");
}] as const);

export const useSidepanelContext = (): SidepanelContext => {
    const context = useContext(SidepanelContext);
    if (!context) {
        throw new Error("SidepanelContext not found");
    }
    return context;
};

export const SidepanelContextProvider = (props: { children: any }) => {
    const [sidepanelState, setSidepanelState] = createSignal<string | null>(null);
    return (
        <SidepanelContext.Provider value={[sidepanelState, setSidepanelState]}>
            {props.children}
        </SidepanelContext.Provider>
    );
};
