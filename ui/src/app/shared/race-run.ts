import { LevelUp } from './level-up';
import { ZoneEntry } from './zone-entry';

export class RaceRun {
    id: number;
    duration_in_seconds: number;
    levels: Array<LevelUp>;
    zones: Array<ZoneEntry>;

    constructor(id: number, seconds: number, levels: Array<LevelUp>, zones: Array<ZoneEntry>) {
        this.duration_in_seconds = seconds;
        this.id = id;
        this.levels = levels;
        this.zones = zones;
    }
}
