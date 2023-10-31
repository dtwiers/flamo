import { ParentComponent } from "solid-js";
import styles from "./editor-panel-row.module.css";

export type EditorPanelRowProps = {
    label: string;
};

export const EditorPanelRow: ParentComponent<EditorPanelRowProps> = (props) => {
    return (
        <>
            <label class={styles.label}>{props.label}</label>
            <div class={styles.value}>{props.children}</div>
        </>
    );
};
