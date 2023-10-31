import { Show, createSignal } from "solid-js";
import { EditorPanelRow } from "../editor-panel-row";
import styles from "./number-editor.module.css";

export type NumberEditorProps = {
    label: string;
    value: number;
    setValue: (value: number) => void;
};

export const NumberEditor = (props: NumberEditorProps) => {
    const [value, setValue] = createSignal(props.value);
    const [isEditing, setIsEditing] = createSignal(false);

    const onSubmit = (e: Event) => {
        e.preventDefault();
        props.setValue(value());
        setIsEditing(false);
    };

    return (
        <EditorPanelRow label={props.label}>
            <Show
                when={isEditing()}
                fallback={
                    <span class={styles.output} onDblClick={() => setIsEditing((p) => !p)}>
                        {value()}
                    </span>
                }
            >
                <form onSubmit={onSubmit}>
                    <input
                        type="number"
                        class={styles.input}
                        value={value()}
                        onChange={(e) => setValue(Number.parseFloat(e.target.value))}
                    />
                </form>
            </Show>
        </EditorPanelRow>
    );
};
