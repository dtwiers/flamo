import { createSignal } from "solid-js";
import { EditorPanel } from "../../../editors/editor-panel";
import { NumberEditor } from "../../../editors/number-editor";
import styles from "./image-editor.module.css";

export type ImageEditorProps = {};

export const ImageEditor = (props: ImageEditorProps) => {
    const [width, setWidth] = createSignal(0);
    const [height, setHeight] = createSignal(0);
    const [quality, setQuality] = createSignal(0);

    return (
        <EditorPanel title="Image Editor">
            <NumberEditor label="Width" value={width()} setValue={setWidth} />
            <NumberEditor label="Height" value={height()} setValue={setHeight} />
            <NumberEditor label="Quality" value={quality()} setValue={setQuality} />
            {/* <div class={styles.imageEditorRow}>
                <label>Width</label>
                <input type="number" />
            </div>
            <div class={styles.imageEditorRow}>
                <label>Height</label>
                <input type="number" />
            </div>
            <div class={styles.imageEditorRow}>
                <label>Background</label>
                <input type="color" />
            </div>
            <div class={styles.quality}>
                <label>Quality</label>
                <input type="range" min="0" max="8000" />
            </div> */}
        </EditorPanel>
    );
};
