export class LevelUp {
    level: number;
    duration_in_seconds: number;

    constructor(level: number, seconds: number) {
        this.level = level;
        this.duration_in_seconds = seconds;
    }
}
