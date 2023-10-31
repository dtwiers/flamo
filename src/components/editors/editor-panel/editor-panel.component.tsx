import { ParentComponent, Show, createMemo, createSignal } from "solid-js";
import styles from "./editor-panel.module.css";

export type EditorPanelProps = {
    title: string;
};

export const EditorPanel: ParentComponent<EditorPanelProps> = (props) => {
    const [isCollapsed, setIsCollapsed] = createSignal(false);
    const collapseText = createMemo(() => (isCollapsed() ? "Expand" : "Collapse"));
    const collapseIcon = createMemo(() => (isCollapsed() ? "▶" : "▼"));
    return (
        <div class={styles.container} classList={{ [styles.collapsed]: isCollapsed() }}>
            <div class={styles.header}>
                <h3 class={styles.title}>{props.title}</h3>
                <button class={styles.collapseButton} type="button" onClick={() => setIsCollapsed((c) => !c)}>
                    {collapseIcon()}
                    <span class="sr-only">{collapseText()}</span>
                </button>
            </div>
            <Show when={!isCollapsed()}>
                <div class={styles.content}>{props.children}</div></Show>
        </div>
    );
};
