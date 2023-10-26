import type { ProjectState } from "../../app/state";
import styles from "./main-window.module.css";
import placeholder from "../../assets/placeholder.jpeg";
import { SidepanelEditor } from "../sidepanel-editor";
import { createSignal, onMount } from "solid-js";
import { clamp } from "../../util/math";

export type MainWindowProps = {
    project: ProjectState | null;
};

export const MainWindow = (props: MainWindowProps) => {
    let previewWindow: HTMLDivElement | undefined = undefined;
    const [scale, setScale] = createSignal(1);
    onMount(() => {
        const SCALE_FACTOR = 0.0015;
        previewWindow!.addEventListener("wheel", (e) => {
            e.preventDefault();
            setScale(s => clamp(s * (1 - SCALE_FACTOR * e.deltaY), 0.01, 10));
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
