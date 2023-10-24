import { For, createSignal } from "solid-js";
import { MainWindow } from "../main-window";
import { useAppState } from "../state-provider";
import { Tab } from "./components/tab";
import styles from "./tabbed-window.module.css";

export type TabbedWindowProps = {};

export const TabbedWindow = (props: TabbedWindowProps) => {
    const state = useAppState();
    const [isEditingName, setEditingName] = createSignal(false);
    // const hasTabs = createMemo(() => state?.appState.projects.ids.length > 0 || true);
    return (
        <div class={styles.tabbedContainer}>
            {/* <Show when={hasTabs()}> */}
            <nav class={styles.navTabs}>
                <For each={state.appState.projects.ids}>
                    {(id) => {
                        const project = state.appState.projects.entities[id];
                        return (
                            <Tab
                                id={id}
                                name={project.name}
                                active={state.appState.currentProjectId === id}
                                onClick={() => {
                                    if (!isEditingName()) {
                                        state.selectProject(id);
                                    }
                                }}
                                onClose={async () => {
                                    await state.closeProject(id);
                                }}
                                onRename={async (name: string) => {
                                    state.setAppState("projects", "entities", id, "name", name);
                                }}
                                onBeginEdit={() => {
                                    setEditingName(true);
                                }}
                                onEndEdit={() => {
                                    setEditingName(false);
                                }}
                            />
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
