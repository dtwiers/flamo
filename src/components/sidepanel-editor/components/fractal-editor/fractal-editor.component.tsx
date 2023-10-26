import styles from "./fractal-editor.module.css";

export type FractalEditorProps = {};

export const FractalEditor = (props: FractalEditorProps) => {
    return (
        <div>
            <h3>Fractal Editor</h3>
            <div class={styles.fractalEditor}>
                <div class={styles.fractalEditorRow}>
                    <label>Affine</label>
                    <button type="button">Edit</button>
                </div>
            </div>
        </div>
    );
};
