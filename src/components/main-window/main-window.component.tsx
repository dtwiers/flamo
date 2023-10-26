import { ProjectState } from "../../app";
import styles from "./main-window.module.css";
import placeholder from "../../assets/placeholder.jpeg";
import { SidepanelEditor } from "../sidepanel-editor";
import { createSignal, onMount } from "solid-js";

export type MainWindowProps = {
    project: ProjectState | null;
};

export const MainWindow = (props: MainWindowProps) => {
    let previewWindow: HTMLDivElement | undefined = undefined;
    const [scale, setScale] = createSignal(1);
    onMount(() => {
        const SCALE_FACTOR = 0.1;
        previewWindow!.addEventListener("wheel", (e) => {
            e.preventDefault();
            if (e.deltaY > 0) {
                setScale(scale() * (1 - SCALE_FACTOR));
            } else {
                setScale(scale() * (1 + SCALE_FACTOR));
            }
        });
    });
    return (
        <div class={styles.mainWindow}>
            <div class={styles.mainContainer}>
                <div ref={previewWindow} class={styles.previewWindow}>
                    <img
                        src={placeholder}
                        alt="placeholder"
                        style={`transform: scale(${scale()});`}
                    />
                    {(scale() * 100).toFixed(1)}%
                </div>
                <SidepanelEditor />
            </div>
            <div class={styles.bottomBar}>BOTTOM BAR</div>
        </div>
    );
};
