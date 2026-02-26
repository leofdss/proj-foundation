//////// Types

class AudioRef {}

type FileRef = AudioRef;

type Result<T, E> =
  | {
      readonly ok: true;
      readonly value: T;
    }
  | {
      readonly ok: false;
      readonly error: E;
    };

//////// Base

type IStore<State> = {
  init(): Promise<Result<void, Error>>;
  getState(): State;
  dispatch(
    handler: (state: State) => Promise<State>,
  ): Promise<Result<void, Error>>;
  subscribe(listener: (state: State) => void): () => void;
};

type IDataBaseEngine<T> = {
  init(): Promise<Result<void, Error>>;
  save(collection: string, id: string, value: T): Promise<Result<void, Error>>;
  get(collection: string, id: string): Promise<Result<T, Error>>;
  getAll(collection: string): Promise<Result<T[], Error>>;
  getAllCollection(): Promise<Result<string[], Error>>;
  update(
    collection: string,
    id: string,
    value: T,
  ): Promise<Result<void, Error>>;
  del(collection: string, id: string): Promise<Result<void, Error>>;
};

type IStorageEngine<T extends FileRef> = IDataBaseEngine<T>;

//////// Project

class Project {
  constructor(
    public readonly id: string,
    public readonly name: string,
    public readonly audios: AudioRef[],
  ) {}
}

//////// Project state

type IProjectState = {
  currentProjectId: string;
};

type IProjectEvent = {};
type IProjectCommand = {};

//////// Player

type IPlayerEngine = {
  init(): Promise<Result<void, Error>>;
  start(audios: AudioRef[]): Promise<Result<void, Error>>;
  play(): Promise<Result<void, Error>>;
  pause(): Promise<Result<void, Error>>;
  stop(): Promise<Result<void, Error>>;
};

//////// Player State

type IPlayerState =
  | {
      kind: "stopped";
    }
  | {
      kind: "playing";
      position: number;
    }
  | {
      kind: "paused";
      position: number;
    };

//////// App

class PlayerService {
  constructor(
    private readonly _playerEngine: IPlayerEngine,
    private readonly _playerStore: IStore<IPlayerState>,
    private readonly _playerStorage: IStorageEngine<AudioRef>,
  ) {}
}

class ProjectService {
  constructor(
    private readonly _projectStore: IStore<IProjectState>,
    private readonly _projectStorage: IStorageEngine<Project>,
  ) {}
}

class PlayerController {
  constructor(private readonly _playerService: PlayerService) {}
}

class ProjectController {
  constructor(private readonly _projectService: ProjectService) {}
}

class PluginController {
  constructor(
    public readonly player: PlayerController,
    public readonly project: ProjectController,
  ) {}
}

class PluginAdminController {}

class App {
  private constructor(
    public readonly player: PlayerController,
    public readonly project: ProjectController,
    public readonly pluginAdmin: PluginAdminController,
    public readonly plugin: PluginController,
  ) {}

  factory(input: {
    playerEngine: IPlayerEngine;
    playerStore: IStore<IPlayerState>;
    playerStorage: IStorageEngine<AudioRef>;
    projectStore: IStore<IProjectState>;
    projectStorage: IStorageEngine<Project>;
  }): App {
    // Services
    const playerService = new PlayerService(
      input.playerEngine,
      input.playerStore,
      input.playerStorage,
    );
    const projectService = new ProjectService(
      input.projectStore,
      input.projectStorage,
    );
    // Controllers
    const playerController = new PlayerController(playerService);
    const projectController = new ProjectController(projectService);
    const pluginController = new PluginController(
      playerController,
      projectController,
    );
    const pluginAdminController = new PluginAdminController();

    //
    return new App(
      playerController,
      projectController,
      pluginAdminController,
      pluginController,
    );
  }
}
