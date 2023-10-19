import { invoke } from "@tauri-apps/api/tauri";
import { createSignal } from "solid-js";
import { createStore, produce } from "solid-js/store";
import { v4 as uuidv4 } from "uuid";
import { ComputeParameters, RenderParameters } from "./state.types";

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

export type RecentProject = {
    name: string;
    path: string;
};

export type AppState = {
    projects: ProjectState[];
    currentProject: ProjectState;
};

export type EntityId = string;

export type EntityState<T> = {
    entities: Record<EntityId, T>;
    ids: EntityId[];
};

export const [projects, setProjects] = createStore<EntityState<ProjectState>>({
    entities: {},
    ids: [],
});
export const [currentProjectId, setCurrentProjectId] = createSignal<
    string | null
>(null);

export const createNewProject = async () => {
    const project: ProjectState = {
        name: "New Project",
        id: uuidv4.generate(),
        temp_path: null,
        saved_project_path: null,
        is_dirty: false,
        progress: null,
        // TODO
        renderParameters: {} as any,
        status: "not_started",
    };
    setProjects(
        produce((state) => {
            state.entities[project.id] = project;
            state.ids.push(project.id);
        })
    );

    // const _server_project = await invoke<ProjectState>("create_new_project", {
    //     project,
    // });
    setCurrentProjectId(project.id);
};

export const closeProject = async (projectId: string) => {
    setProjects(
        produce((state) => {
            delete state.entities[projectId];
            state.ids = state.ids.filter((id) => id !== projectId);
        })
    );
    if (currentProjectId() === projectId) {
        setCurrentProjectId(projects.ids.length > 0 ? projects.ids[0] : null);
    }
};

export const modifyVariationValue = <
    K extends keyof ComputeParameters,
    K2 extends keyof ComputeParameters[K],
    V extends ComputeParameters[K][K2]
>(
    projectId: string,
    key: K,
    key2: K2,
    value: V
) => {
    setProjects(
        produce((state) => {
            state.entities[projectId].renderParameters.computeParameters[key][
                key2
            ] = value;
        })
    );
};
