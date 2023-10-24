import { Show, createSignal } from "solid-js";
import styles from "./tab.module.css";

export type TabProps = {
    name: string;
    id: string;
    active: boolean;
    onClick: () => void;
    onClose: () => Promise<void>;
    onRename: (name: string) => Promise<void>;
    onBeginEdit: () => void;
    onEndEdit: () => void;
};

export const Tab = (props: TabProps) => {
    const [isEditingName, setEditingName] = createSignal(false);
    const [name, setName] = createSignal(props.name);
    const setFocus = (element: HTMLElement) => setTimeout(() => element.focus(), 0);
    return (
        <div class={styles.tab}>
            <Show
                when={isEditingName()}
                fallback={
                    <button
                        class={styles.tabButton}
                        classList={{
                            [styles.active]: props.active,
                        }}
                        type="button"
                        onClick={props.onClick}
                        onDblClick={async () => {
                            if (props.active) {
                                props.onBeginEdit();
                                setEditingName((prev) => !prev);
                            }
                        }}
                    >
                        {name()}
                    </button>
                }
            >
                <form
                    classList={{ [styles.tabButton]: true, [styles.active]: true }}
                    onSubmit={() => {
                        setEditingName(false);
                        props.onEndEdit();
                        props.onRename(name());
                    }}
                >
                    <input
                        ref={setFocus}
                        type="text"
                        value={name()}
                        onInput={(e) => {
                            setName(e.currentTarget.value);
                        }}
                        onBlur={() => {
                            setName(props.name);
                            setEditingName(false);
                            props.onEndEdit();
                        }}
                        onKeyDown={(e) => {
                            if (e.key === "Escape") {
                                setName(props.name);
                                setEditingName(false);
                                props.onEndEdit();
                            }
                        }}
                    />
                </form>
            </Show>
            <Show when={props.active}>
                <button class={styles.tabClose} onClick={props.onClose}>
                    <svg height="12" width="12" viewBox="0 0 30 30">
                        <line x1="0" y1="0" x2="30" y2="30" stroke="black" stroke-width="10" />
                        <line x1="0" y1="30" x2="30" y2="0" stroke="black" stroke-width="10" />
                    </svg>
                </button>
            </Show>
        </div>
    );
};
