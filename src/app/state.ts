// import { invoke } from "@tauri-apps/api/tauri";
import { createStore, produce } from "solid-js/store";
import { v4 as uuidv4 } from "uuid";
import { ComputeParameters, RenderParameters } from "./state.types";
import { makeComputeParametersUpdates } from "./compute-parameters";

export type ProjectState = {
    name: string;
    id: string;
    temp_path: string | null;
    saved_project_path: string | null;
    is_dirty: boolean;
    progress: number | null;
    status: ProjectStatus;
    renderParameters: RenderParameters;
};

export type ProjectStatus = "not_started" | "in_progress" | "complete";

// LATER
export type RecentProject = {
    name: string;
    path: string;
};

export type AppState = {
    projects: EntityState<ProjectState>;
    currentProjectId: string | null;
    readonly currentProject: ProjectState | null;
};

export type EntityId = string;

export type EntityState<T> = {
    entities: Record<EntityId, T>;
    ids: EntityId[];
};

export const createAppState = () => {
    const [appState, setAppState] = createStore<AppState>({
        projects: {
            entities: {},
            ids: [],
        },
        currentProjectId: null,
        get currentProject() {
            if (!this.currentProjectId) return null;
            return this.projects.entities[this.currentProjectId];
        },
    });

    const createNewProject = async () => {
        console.log("createNewProject");
        const project: ProjectState = {
            name: "New Project",
            id: uuidv4(),
            temp_path: null,
            saved_project_path: null,
            is_dirty: false,
            progress: null,
            // TODO
            renderParameters: {} as any,
            status: "not_started",
        };
        setAppState(
            produce((state) => {
                state.projects.entities[project.id] = project;
                state.projects.ids.push(project.id);
                state.currentProjectId = project.id;
            }),
        );
    };

    const closeProject = async (projectId: string) => {
        setAppState(
            produce((state) => {
                const index = state.projects.ids.indexOf(projectId);
                delete state.projects.entities[projectId];
                state.projects.ids = state.projects.ids.filter((id) => id !== projectId);
                if (state.currentProjectId === projectId) {
                    state.currentProjectId =
                        state.projects.ids.length > 0
                            ? state.projects.ids[Math.min(index, state.projects.ids.length - 1)]
                            : null;
                }
            }),
        );
    };

    const modifyVariationValue = <
        K extends keyof ComputeParameters,
        K2 extends keyof ComputeParameters[K],
        V extends ComputeParameters[K][K2],
    >(
        projectId: string,
        key: K,
        key2: K2,
        value: V,
    ) => {
        setAppState(
            produce((state) => {
                state.projects.entities[projectId].renderParameters.computeParameters[key][key2] =
                    value;
            }),
        );
    };

    const modifyComputeParameters = makeComputeParametersUpdates(appState, setAppState);

    const selectProject = (projectId: string) => {
        setAppState(
            produce((state) => {
                state.currentProjectId = projectId;
            }),
        );
    };

    return {
        appState,
        setAppState,
        createNewProject,
        selectProject,
        closeProject,
        modifyVariationValue,
        modifyComputeParameters,
    };
};
