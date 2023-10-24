import { For, Show, createMemo } from "solid-js";
import { useAppState } from "../state-provider";
import { MainWindow } from "../main-window";
import styles from "./tabbed-window.module.css";

export type TabbedWindowProps = {};

export const TabbedWindow = (props: TabbedWindowProps) => {
    const state = useAppState();
    // const hasTabs = createMemo(() => state?.appState.projects.ids.length > 0 || true);
    return (
        <div class={styles.tabbedContainer}>
            {/* <Show when={hasTabs()}> */}
            <nav class={styles.navTabs}>
                <For each={state.appState.projects.ids}>
                    {(id) => {
                        const project = state.appState.projects.entities[id];
                        return (
                            <button
                                class={styles.navTab}
                                classList={{
                                    [styles.active]: state.appState.currentProjectId === id,
                                }}
                                onClick={() => {
                                    state.selectProject(id);
                                }}
                            >
                                {project.name}
                            </button>
                        );
                    }}
                </For>
                <button
                    type="button"
                    class={styles.addButton}
                    onClick={() => {
                        state?.createNewProject();
                    }}
                >
                    +
                </button>
            </nav>
            {/* </Show> */}
            <MainWindow project={state.appState.currentProject} />
        </div>
    );
};
