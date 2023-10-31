import { EditorPanel } from "../../../editors/editor-panel";
import styles from "./fractal-editor.module.css";

export type FractalEditorProps = {};

export const FractalEditor = (props: FractalEditorProps) => {
    return (
        <EditorPanel title="Fractal Editor">
            <div class={styles.fractalEditor}>
                <div class={styles.fractalEditorRow}>
                    <label>Affine</label>
                    <button type="button">Edit</button>
                </div>
            </div>
        </EditorPanel>
    );
};
