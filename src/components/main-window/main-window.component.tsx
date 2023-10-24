import { ProjectState } from "../../app";
import styles from "./main-window.module.css";

export type MainWindowProps = {
    project: ProjectState | null;
};

export const MainWindow = (props: MainWindowProps) => {
    return (
        <div class={styles.mainWindow}>
            {props.project?.id}
        </div>
    );
};
