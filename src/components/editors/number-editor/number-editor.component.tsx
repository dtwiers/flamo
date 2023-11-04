import { Show, createMemo, createSignal } from "solid-js";
import { EditorPanelRow } from "../editor-panel-row";
import styles from "./number-editor.module.css";
import { useSidepanelContext } from "../../sidepanel-editor/sidepanel-context";
import { setFocus } from "../../../util/dom";
import { set } from "zod";

export type NumberEditorProps = {
    id: string;
    label: string;
    value: number;
    setValue: (value: number) => void;
};

export const NumberEditor = (props: NumberEditorProps) => {
    const [value, setValue] = createSignal(props.value);
    const [editingField, setEditingField] = useSidepanelContext();
    const isEditing = createMemo(() => editingField() === props.id);

    const onSubmit = (e: Event) => {
        e.preventDefault();
        props.setValue(value());
        setEditingField(null);
    };

    const onBlur = () => {
        setEditingField(null);
    };

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key === "Escape") {
            setValue(props.value);
            setEditingField(null);
        }
    };

    return (
        <EditorPanelRow label={props.label}>
            <Show
                when={isEditing()}
                fallback={
                    <span
                        class={styles.output}
                        onDblClick={() => {
                            console.log(props.id);
                            setEditingField(props.id);
                        }}
                    >
                        {value()}
                    </span>
                }
            >
                <form onSubmit={onSubmit}>
                    <input
                        ref={setFocus}
                        type="number"
                        class={styles.input}
                        value={value()}
                        onChange={(e) => setValue(Number.parseFloat(e.target.value))}
                        onBlur={onBlur}
                        onKeyDown={onKeyDown}
                    />
                </form>
            </Show>
        </EditorPanelRow>
    );
};
