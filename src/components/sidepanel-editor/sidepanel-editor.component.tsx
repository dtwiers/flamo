import { Match, Switch, createSignal } from "solid-js";
import styles from "./sidepanel-editor.module.css";
import { ImageEditor } from "./components/image-editor";
import { FractalEditor } from "./components/fractal-editor";
import { useAppState } from "../state-provider";

export type SidepanelEditorProps = {};

export const SidepanelEditor = () => {
    const [isOpen, setIsOpen] = createSignal(true);
    const state = useAppState();
    return (
        <Switch>
            <Match when={isOpen()}>
                <div class={styles.open}>
                    <div class={styles.header}>
                        <h2>{state.appState.currentProject?.name} Properties</h2>
                        <button type="button" onClick={() => setIsOpen(false)}>
                            Close
                        </button>
                    </div>
                    <ImageEditor />
                    <FractalEditor />
                </div>
            </Match>
            <Match when={!isOpen()}>
                <button type="button" class={styles.closed} onClick={() => setIsOpen(true)}>
                    I am Closed!
                </button>
            </Match>
        </Switch>
    );
};
