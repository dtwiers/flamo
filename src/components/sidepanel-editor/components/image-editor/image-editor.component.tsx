import styles from './image-editor.module.css';

export type ImageEditorProps = {};

export const ImageEditor = (props: ImageEditorProps) => {
    return (<div>
        <h3>Image Editor</h3>
        <div class={styles.imageEditor}>
            <div class={styles.imageEditorRow}>
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
            </div>
        </div>
    </div>);
}
